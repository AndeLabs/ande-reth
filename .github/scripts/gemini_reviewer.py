
import os
import requests
import sys

try:
    import google.generativeai as genai
except ImportError:
    print("Error: google-generativeai not installed. Please install with: pip install google-generativeai")
    sys.exit(1)

# --- Constantes y Configuración ---

# Obtener las claves API y la URL del PR desde las variables de entorno
GEMINI_API_KEY = os.getenv('GEMINI_API_KEY')
GITHUB_TOKEN = os.getenv('GITHUB_TOKEN')
PR_URL = os.getenv('PR_URL')

# Verificar que las variables de entorno necesarias estén presentes
if not all([GEMINI_API_KEY, GITHUB_TOKEN, PR_URL]):
    print("Error: Faltan una o más variables de entorno (GEMINI_API_KEY, GITHUB_TOKEN, PR_URL).")
    sys.exit(1)

# Configurar el cliente de la API de Gemini
genai.configure(api_key=GEMINI_API_KEY)

# Configurar los encabezados para las solicitudes a la API de GitHub
GITHUB_HEADERS = {
    'Authorization': f'token {GITHUB_TOKEN}',
    'Accept': 'application/vnd.github.v3.diff'
}

COMMENTS_URL = f'{PR_URL}/comments'

# --- Funciones Principales ---

def get_pr_diff():
    """Obtiene el diff del Pull Request desde la API de GitHub."""
    if not PR_URL:
        print("Error: PR_URL no está definido")
        sys.exit(1)
        
    try:
        response = requests.get(PR_URL, headers=GITHUB_HEADERS)
        response.raise_for_status() # Lanza una excepción para respuestas de error (4xx o 5xx)
        return response.text
    except requests.exceptions.RequestException as e:
        print(f"Error al obtener el diff del PR: {e}")
        sys.exit(1)

def get_gemini_review(diff):
    """Envía el diff a Gemini y obtiene una revisión de código."""
    if not diff:
        print("El diff está vacío. No hay nada que revisar.")
        return None

    # Prompt para Gemini, inspirado en el workflow de Claude
    prompt = f"""
    Por favor, actúa como un experto revisor de código. Revisa el siguiente Pull Request y proporciona feedback sobre:
    - Calidad del código y buenas prácticas.
    - Potenciales bugs o problemas.
    - Consideraciones de rendimiento.
    - Brechas de seguridad.
    - Cobertura de tests.

    Sé constructivo, claro y útil en tu feedback. Formatea tu respuesta en Markdown.

    Aquí está el diff del Pull Request:
    ```diff
    {diff}
    ```
    """

    try:
        model = genai.GenerativeModel('gemini-1.5-flash') # Usamos flash por su velocidad y eficiencia
        response = model.generate_content(prompt)
        return response.text
    except Exception as e:
        print(f"Error al contactar la API de Gemini: {e}")
        sys.exit(1)

def post_github_comment(comment):
    """Publica un comentario en el Pull Request."""
    if not comment:
        print("No se generó ningún comentario para publicar.")
        return

    payload = {'body': comment}
    headers = {
        'Authorization': f'token {GITHUB_TOKEN}',
        'Accept': 'application/vnd.github.v3+json'
    }
    response = None
    try:
        response = requests.post(COMMENTS_URL, json=payload, headers=headers)
        response.raise_for_status()
        print("Comentario de revisión publicado exitosamente en GitHub.")
    except requests.exceptions.RequestException as e:
        print(f"Error al publicar el comentario en GitHub: {e}")
        if response and hasattr(response, 'text'):
            print(f"Response body: {response.text}")
        sys.exit(1)

# --- Flujo de Ejecución ---

if __name__ == "__main__":
    print("Iniciando el proceso de revisión de código con Gemini...")
    
    # 1. Obtener el diff del PR
    pr_diff = get_pr_diff()
    
    # 2. Obtener la revisión de Gemini
    review_comment = get_gemini_review(pr_diff)
    
    # 3. Publicar el comentario en GitHub
    post_github_comment(review_comment)
    
    print("Proceso de revisión finalizado.")
