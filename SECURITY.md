# Security Policy

## Scope

Security reports should cover vulnerabilities in FrameStack source code, build tooling, documentation examples, generated projects, and maintained integrations.

## Reporting

Please use GitHub's **Report a vulnerability** feature for this repository when it is available. Do not publish exploitable details in a public issue.

If private vulnerability reporting is unavailable, contact the repository maintainers privately through the Cybercore Tech GitHub organization and request a secure reporting channel.

## Expectations

- Do not include credentials, tokens, or private user data in reports.
- Include affected versions or commit references, reproduction steps, and impact.
- Allow maintainers reasonable time to investigate and prepare a fix before public disclosure.

## Security design principles

FrameStack should favor least privilege, explicit capabilities, safe defaults, dependency transparency, and reproducible builds. Integrations must not silently broaden the permissions of the core runtime.
