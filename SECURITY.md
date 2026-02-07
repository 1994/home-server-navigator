# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of Home Server Navigator seriously. If you believe you have found a security vulnerability, please report it to us as described below.

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to [your-email@example.com] (replace with your actual security contact).

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information in your report:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

## Security Considerations

This application is designed for **home server environments** and should be used with the following security considerations:

### Network Security
- **Do not expose directly to the public internet** without additional protection
- Run behind a reverse proxy (nginx, Traefik, etc.) with HTTPS if remote access is needed
- Consider using a VPN or private network for access

### Access Control
- Currently no built-in authentication (designed for trusted local networks)
- systemd service runs with system privileges for service discovery
- Data file permissions should be restricted to the service user

### Data Storage
- Service data is stored in JSON format locally
- No sensitive data (passwords, tokens) should be stored in service configuration
- Locked fields prevent automatic overwrites but don't provide cryptographic protection

## Security Best Practices

1. **Run in a trusted network environment**
2. **Use a firewall** to restrict access to the web UI port
3. **Keep the software updated** to the latest version
4. **Review discovered services** regularly for unauthorized additions
5. **Use HTTPS** when accessing remotely (via reverse proxy)

## Acknowledgments

We thank the following individuals for their security contributions:

*None yet - be the first!*
