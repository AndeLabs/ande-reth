import os
import sys
import argparse
import requests
from datetime import datetime

def get_workflow_runs(github_token, repo, workflow_name="docker.yml", limit=5):
    """Get recent workflow runs for analysis"""
    url = f"https://api.github.com/repos/{repo}/actions/workflows/{workflow_name}/runs"
    headers = {
        'Authorization': f'token {github_token}',
        'Accept': 'application/vnd.github.v3+json'
    }
    
    params = {'per_page': limit, 'status': 'failure'}
    
    try:
        response = requests.get(url, headers=headers, params=params)
        response.raise_for_status()
        return response.json().get('workflow_runs', [])
    except Exception as e:
        print(f"Error fetching workflow runs: {e}")
        return []

def analyze_workflow_with_gemini(workflow_data, github_token, repo):
    """Analyze specific workflow run using Gemini"""
    
    api_key = os.getenv('GEMINI_API_KEY')
    if not api_key:
        print("GEMINI_API_KEY not found")
        return
    
    try:
        import google.generativeai as genai
        genai.configure(api_key=api_key)
    except ImportError:
        print("google-generativeai not available")
        return
    
    # Get workflow logs
    logs_url = workflow_data['logs_url']
    headers = {
        'Authorization': f'token {github_token}',
        'Accept': 'application/vnd.github.v3+json'
    }
    
    try:
        logs_response = requests.get(logs_url, headers=headers)
        if logs_response.status_code != 200:
            print(f"Could not fetch logs: {logs_response.status_code}")
            return
            
        logs_content = logs_response.text
        
    except Exception as e:
        print(f"Error fetching logs: {e}")
        return
    
    # Prepare analysis prompt
    prompt = f"""
    Analiza este fallo del workflow Docker para el proyecto ev-reth (cliente Ethereum).
    
    Información del Workflow:
    - ID: {workflow_data['id']}
    - Evento: {workflow_data['event']}
    - Rama: {workflow_data['head_branch']}
    - Commit: {workflow_data['head_sha']}
    - Status: {workflow_data['conclusion']}
    - Fecha: {workflow_data['created_at']}
    
    Por favor, analiza los logs y proporciona:
    1. **Problema Principal**: ¿Cuál es la causa raíz del fallo?
    2. **Errores Específicos**: Lista los mensajes de error clave
    3. **Solución Recomendada**: Pasos exactos para corregirlo
    4. **Prevención**: Cómo evitar este problema en el futuro
    5. **Impacto**: ¿Qué partes del sistema están afectadas?
    
    Responde en español y sé específico con comandos y cambios de código necesarios.
    
    Logs del Workflow:
    {logs_content[:10000]}  # Limit to first 10k chars for context
    """
    
    try:
        model = genai.GenerativeModel('gemini-1.5-flash')
        response = model.generate_content(prompt)
        
        return response.text
        
    except Exception as e:
        print(f"Error analyzing with Gemini: {e}")
        return None

def post_issue_analysis(repo, analysis, workflow_data):
    """Create GitHub issue with analysis"""
    
    github_token = os.getenv('GITHUB_TOKEN')
    if not github_token:
        print("GITHUB_TOKEN not found")
        return
    
    url = f"https://api.github.com/repos/{repo}/issues"
    headers = {
        'Authorization': f'token {github_token}',
        'Accept': 'application/vnd.github.v3+json'
    }
    
    title = f"🐳 Docker Build Failure Analysis - Workflow #{workflow_data['id']}"
    
    body = f"""## Análisis de Fallo Docker Workflow

**Información del Workflow:**
- ID: {workflow_data['id']}
- Evento: {workflow_data['event']}
- Rama: {workflow_data['head_branch']}
- Commit: {workflow_data['head_sha']}
- Status: {workflow_data['conclusion']}
- Fecha: {workflow_data['created_at']}

## 🤖 Análisis con Gemini AI

{analysis}

## 📋 Acciones Recomendadas

1. [ ] Revisar los problemas identificados arriba
2. [ ] Implementar las soluciones sugeridas
3. [ ] Testear los cambios localmente
4. [ ] Hacer push de las correcciones

---
*Este issue fue creado automáticamente usando Google Gemini AI*
*Generado: {datetime.now().isoformat()}*
"""
    
    payload = {
        'title': title,
        'body': body,
        'labels': ['docker', 'ci/cd', 'automated-analysis', 'bug']
    }
    
    try:
        response = requests.post(url, json=payload, headers=headers)
        response.raise_for_status()
        
        issue_data = response.json()
        print(f"Issue creado: #{issue_data['number']} - {issue_data['html_url']}")
        return issue_data
        
    except Exception as e:
        print(f"Error creating issue: {e}")
        return None

def main():
    parser = argparse.ArgumentParser(description='Analyze Docker workflow failures with Gemini')
    parser.add_argument('--repo', default='AndeLabs/ande-reth', help='GitHub repository')
    parser.add_argument('--workflow', default='docker.yml', help='Workflow filename')
    parser.add_argument('--limit', type=int, default=3, help='Number of recent failures to analyze')
    parser.add_argument('--create-issue', action='store_true', help='Create GitHub issue with analysis')
    
    args = parser.parse_args()
    
    github_token = os.getenv('GITHUB_TOKEN')
    if not github_token:
        print("GITHUB_TOKEN not found in environment")
        sys.exit(1)
    
    print(f"🔍 Analizando fallos recientes del workflow {args.workflow}...")
    
    # Get recent failed workflow runs
    failed_runs = get_workflow_runs(github_token, args.repo, args.workflow, args.limit)
    
    if not failed_runs:
        print("No se encontraron workflows fallidos recientes")
        return
    
    print(f"📊 Encontrados {len(failed_runs)} workflows fallidos")
    
    for i, run in enumerate(failed_runs, 1):
        print(f"\n🔍 Analizando workflow #{run['id']} ({i}/{len(failed_runs)})")
        
        analysis = analyze_workflow_with_gemini(run, github_token, args.repo)
        
        if analysis:
            print(f"\n📝 Análisis Gemini para Workflow #{run['id']}:")
            print("=" * 60)
            print(analysis)
            print("=" * 60)
            
            if args.create_issue:
                print(f"\n🎫 Creando issue para workflow #{run['id']}...")
                issue = post_issue_analysis(args.repo, analysis, run)
                if issue:
                    print(f"✅ Issue creado: #{issue['number']}")
        else:
            print(f"❌ No se pudo analizar el workflow #{run['id']}")
    
    print(f"\n✅ Análisis completado")

if __name__ == "__main__":
    main()