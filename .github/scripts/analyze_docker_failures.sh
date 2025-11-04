#!/bin/bash

# Script para analizar fallos de Docker con Gemini AI
# Uso: ./analyze_docker_failures.sh [--create-issue] [--limit N]

set -e

# Default values
CREATE_ISSUE=false
LIMIT=3
REPO="${GITHUB_REPOSITORY:-AndeLabs/ande-reth}"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --create-issue)
            CREATE_ISSUE=true
            shift
            ;;
        --limit)
            LIMIT="$2"
            shift 2
            ;;
        --repo)
            REPO="$2"
            shift 2
            ;;
        -h|--help)
            echo "Uso: $0 [--create-issue] [--limit N] [--repo REPO]"
            echo ""
            echo "Opciones:"
            echo "  --create-issue    Crear GitHub issue con el análisis"
            echo "  --limit N         Número de fallos recientes a analizar (default: 3)"
            echo "  --repo REPO       Repositorio GitHub (default: AndeLabs/ande-reth)"
            echo "  -h, --help        Mostrar esta ayuda"
            exit 0
            ;;
        *)
            echo "Opción desconocida: $1"
            exit 1
            ;;
    esac
done

# Check environment
if [ -z "$GITHUB_TOKEN" ]; then
    echo "❌ Error: GITHUB_TOKEN no está configurado"
    echo "   Exporta la variable: export GITHUB_TOKEN=your_token"
    exit 1
fi

if [ -z "$GEMINI_API_KEY" ]; then
    echo "❌ Error: GEMINI_API_KEY no está configurada"
    echo "   Exporta la variable: export GEMINI_API_KEY=your_key"
    exit 1
fi

echo "🔍 Analizando fallos de Docker para $REPO..."
echo "   Límite: $LIMIT workflows"
echo "   Crear issue: $CREATE_ISSUE"
echo ""

# Change to script directory
cd "$(dirname "$0")"

# Check if Python script exists
if [ ! -f "analyze_docker_failures.py" ]; then
    echo "❌ Error: No se encuentra analyze_docker_failures.py"
    exit 1
fi

# Check Python dependencies
python3 -c "import google.generativeai, requests" 2>/dev/null || {
    echo "❌ Error: Dependencias Python no instaladas"
    echo "   Instala con: pip install -r requirements.txt"
    exit 1
}

# Run analysis
ARGS="--repo $REPO --limit $LIMIT"
if [ "$CREATE_ISSUE" = "true" ]; then
    ARGS="$ARGS --create-issue"
fi

python3 analyze_docker_failures.py $ARGS

echo ""
echo "✅ Análisis completado"