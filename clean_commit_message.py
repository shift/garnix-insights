#!/usr/bin/env python3
import sys
import re

message = sys.stdin.read()

# Replace all Unicode emojis with ASCII equivalents
replacements = {
    '🚀': '[ROCKET]',
    '✅': '[OK]', 
    '❌': '[FAIL]',
    '🚫': '[CANCELLED]',
    '❓': '[UNKNOWN]',
    '🔍': '[SEARCH]',
    '🎉': '[SUCCESS]', 
    '👍': '[GOOD]',
    '⚠️': '[WARNING]',
    '📊': '[STATS]',
    '🏗️': '[BUILD]',
    '📚': '[DOCS]',
    '🎯': '[TARGET]',
    '💥': '[BREAKING]',
    '📖': '[DOCUMENTATION]',
    '🔧': '[MAINTENANCE]',
    '♻️': '[REFACTOR]',
    '✨': '[FEATURE]',
    '🐛': '[BUG]',
    '🔒': '[SECURE]',
    '🔄': '[REFRESH]',
    '🎨': '[STYLE]',
    '🚨': '[ALERT]',
    '📦': '[PACKAGE]'
}

for emoji, replacement in replacements.items():
    message = message.replace(emoji, replacement)

# Remove any remaining Unicode characters > 127
cleaned = ''.join(char if ord(char) <= 127 else f'[U+{ord(char):04X}]' for char in message)
print(cleaned, end='')
