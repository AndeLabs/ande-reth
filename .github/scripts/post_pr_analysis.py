import os
import sys
import argparse
import requests

def post_pr_analysis(pr_number):
    """Post Docker failure analysis to PR"""
    
    github_token = os.getenv('GITHUB_TOKEN')
    if not github_token:
        print("GITHUB_TOKEN not found")
        return
    
    # Read analysis file
    try:
        with open('docker_failure_analysis.md', 'r') as f:
            analysis_content = f.read()
    except FileNotFoundError:
        print("Analysis file not found")
        return
    
    # Get PR details
    repo = os.getenv('GITHUB_REPOSITORY', 'AndeLabs/ande-reth')
    comments_url = f"https://api.github.com/repos/{repo}/issues/{pr_number}/comments"
    
    headers = {
        'Authorization': f'token {github_token}',
        'Accept': 'application/vnd.github.v3+json'
    }
    
    payload = {
        'body': f"""## 🐳 Docker Build Failure Analysis

The automated Docker build failed. Here's the analysis:

{analysis_content}

---
*This analysis was generated automatically using Google Gemini AI.*
"""
    }
    
    try:
        response = requests.post(comments_url, json=payload, headers=headers)
        response.raise_for_status()
        print(f"Analysis posted to PR #{pr_number}")
    except Exception as e:
        print(f"Error posting to PR: {e}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Post Docker analysis to PR')
    parser.add_argument('--pr', required=True, type=int, help='PR number')
    args = parser.parse_args()
    
    post_pr_analysis(args.pr)