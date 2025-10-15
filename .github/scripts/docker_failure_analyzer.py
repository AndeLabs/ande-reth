import os
import sys
import json
import glob

try:
    import google.generativeai as genai
except ImportError:
    print("Error: google-generativeai not installed")
    sys.exit(1)

def analyze_docker_logs():
    """Analyze Docker build failure logs using Gemini"""
    
    # Find log files from failed build
    log_files = glob.glob("*.log") + glob.glob("build-*.txt") + glob.glob("docker-*.log")
    
    if not log_files:
        print("No log files found for analysis")
        return
    
    # Read all log content
    log_content = ""
    for log_file in log_files:
        try:
            with open(log_file, 'r') as f:
                log_content += f"\n=== {log_file} ===\n"
                log_content += f.read() + "\n"
        except Exception as e:
            print(f"Error reading {log_file}: {e}")
    
    if not log_content.strip():
        print("No content found in log files")
        return
    
    # Configure Gemini
    api_key = os.getenv('GEMINI_API_KEY')
    if not api_key:
        print("GEMINI_API_KEY not found")
        return
        
    genai.configure(api_key=api_key)
    
    # Analysis prompt
    prompt = f"""
    Analyze this Docker build failure for a Rust project (ev-reth Ethereum client).
    
    Context:
    - This is a Rust project using cargo-chef for optimized Docker builds
    - The project includes multiple crates and uses jemalloc
    - Cross-compilation for multiple architectures (amd64, arm64)
    
    Please analyze these build logs and provide:
    1. Root cause of the failure
    2. Specific error messages and their meaning
    3. Recommended fixes with exact commands/code changes
    4. Prevention strategies for future builds
    
    Focus on Docker, Rust compilation, and cross-compilation issues.
    
    Build Logs:
    {log_content}
    """
    
    try:
        model = genai.GenerativeModel('gemini-1.5-flash')
        response = model.generate_content(prompt)
        
        # Save analysis
        with open('docker_failure_analysis.md', 'w') as f:
            f.write("# Docker Build Failure Analysis\n\n")
            f.write(response.text)
        
        print("Docker failure analysis completed")
        print(response.text)
        
    except Exception as e:
        print(f"Error analyzing with Gemini: {e}")

if __name__ == "__main__":
    analyze_docker_logs()