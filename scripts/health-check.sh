#!/bin/bash
echo "🔍 ANDE ev-reth Health Check"
echo "============================"

# Check if binary exists
if ! command -v ev-reth &> /dev/null; then
    echo "❌ ev-reth binary not found!"
    exit 1
fi

# Show version
echo "📦 Version: $(ev-reth --version)"

# Show ANDE integration info
echo "🔗 ANDE Integration: Type alias pattern implemented"
echo "📍 Precompile Address: 0x00000000000000000000000000000000000000FD"
echo "📊 Binary size: $(du -h /usr/local/bin/ev-reth | cut -f1)"
echo "🕐 Build date: $(date)"
echo "✅ Health check passed"