# Security Policy

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability, please follow these steps:

### Private Disclosure

**Do not create a public GitHub issue for security vulnerabilities.**

Instead, please report security issues privately:

1. **Email:** Send details to shift@someone.section.me
2. **Subject:** Include "SECURITY" in the subject line
3. **Details:** Provide as much information as possible:
   - Description of the vulnerability
   - Steps to reproduce the issue
   - Potential impact
   - Suggested fix (if you have one)

### What to Expect

- **Acknowledgment:** We'll acknowledge receipt within 24 hours
- **Initial Assessment:** We'll provide an initial assessment within 72 hours
- **Regular Updates:** We'll keep you informed of our progress
- **Resolution Timeline:** We aim to resolve critical issues within 14 days
### Security Measures

This project implements several security measures:

#### Code Security
- **No unsafe code:** Rust's `unsafe_code` is forbidden via lints
- **Dependency auditing:** Regular security audits via `cargo audit`
- **License compliance:** Automated license checking via `cargo deny`
- **Supply chain security:** Pinned dependencies with lock file

#### Network Security
- **TLS enforcement:** All HTTP communications use HTTPS
- **JWT token handling:** Secure token management for Garnix API
- **Input validation:** All external inputs are validated
- **Rate limiting:** Built-in protections against abuse

#### Development Security
- **Signed commits:** Encouraged for maintainers
- **Branch protection:** Main branch protected with required reviews
- **Automated security scanning:** GitHub security advisories enabled
- **Vulnerability alerts:** Automated dependency vulnerability notifications

### Disclosure Process

When we receive a security report:

1. **Triage:** Assess the severity and impact
2. **Investigation:** Reproduce and understand the issue
3. **Fix Development:** Create a patch in a private branch
4. **Testing:** Thoroughly test the fix
5. **Coordination:** Work with the reporter on disclosure timeline
6. **Release:** Publish the fix and security advisory
7. **Attribution:** Credit the reporter (with their permission)

### Security Best Practices

When using Garnix Insights:

#### JWT Token Security
- Store JWT tokens securely (use environment variables)
- Rotate tokens regularly
- Don't commit tokens to version control
- Use minimal permission scopes when possible

#### Network Security
- Use HTTPS for all server deployments
- Implement proper firewall rules
- Consider rate limiting for public deployments
- Monitor logs for suspicious activity

#### MCP Server Security
- Run MCP server in isolated environments when possible
- Validate all MCP client requests
- Implement proper access controls
- Monitor resource usage

## Bug Bounty

We don't currently offer a formal bug bounty program, but we greatly appreciate security research and will:

- Publicly acknowledge your contribution (with permission)
- Provide swag or small tokens of appreciation when possible
- Work with you on coordinated disclosure

## Contact

For security-related questions or concerns:
- **Email:** shift@someone.section.me
- **GPG Key:** Available upon request
- **Response Time:** Within 24 hours for security issues

Thank you for helping keep Garnix Insights and its users safe!
