# DirectDNS.net - Preview Your Website Without Updating DNS

### Overview

DirectDNS.net enables seamless preview and testing of websites before making DNS changes. Imagine your web server is hosted at the IP address **45.8.22.46**, and you've set up configurations for **example.com** that aren’t yet linked to this IP. DirectDNS allows you to preview your website as if the DNS is already updated.

By entering the **IP address** (e.g., 45.8.22.46) and **hostname** (e.g., example.com), DirectDNS creates a reverse proxy, simulating a live environment where the server behaves as though the hostname is already resolving to the desired IP. This is ideal for:

- **Testing:** Verify configurations for websites and applications in a realistic environment.  
- **Troubleshooting:** Diagnose issues prior to DNS propagation.  
- **Demonstrations:** Showcase projects to stakeholders without waiting for DNS updates.

For every session, it generates a unique subdomain in the format `<uuid>.directdns.net`, which acts as the URL for the reverse proxy. You can also skip specifying an IP address if you simply want to bypass cache and view a clean version of your site.

This is aimed to be a free, open source alternative to websites like SkipDNS.link and WithoutDNS.com.

### Key Benefits
- Instant website preview under your desired hostname or subdomain.
- Dynamically generated subdomains for testing individual setups.
- Avoid downtime or propagation delays during DNS changes.
- Free and unlimited reverse proxies without authentication being necessary.

### Technical Stack
DirectDNS is powered by a **Rust** backend and a **React** frontend. In addition, UUIDs are linked to hostnames and IP addresses, which is stored in a **MySQL** database. As the project is still evolving, there may be opportunities for further code optimizations.

### API
At the moment, the only endpoint is POST `/api/create` with the `ip_address` and `hostname` parameters. This will return a `full_url`, which is the full URL of the reverse proxy that is generated.

### To-do List
- [ ] Clean up and update libraries to the latest versions.
- [ ] Add an option to select the amount of time that a proxy is active for.
- [ ] Add some way for the user to delete reverse proxies.
- [ ] Add caching for the UUIDs.
- [ ] Improve documentation and add instructions for self-hosting.
- [ ] Integrate with the cPanel website hosting control panel.
- [ ] Add authentication so that users can manage and store their reverse proxies.
