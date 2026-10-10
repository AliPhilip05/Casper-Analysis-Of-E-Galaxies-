# APhlip.org

A self-contained static personal website for Ali Philip. It lives in its own folder so it can be maintained or hosted separately from the Casper analysis project.

## Pages

- `index.html` — portfolio home and three selected projects
- `research.html` — the multi-band E+A galaxy CAS project
- `coursework.html` — the CSC 326 assignment and independent project notes

## Files

- `styles.css` — shared responsive styles
- `site.js` — mobile navigation and current footer year

## Free hosting on GitHub Pages

The repository's `.github/workflows/deploy-pages.yml` workflow publishes this website to GitHub Pages when its source changes are pushed to `main`. It places the portfolio at the Pages root and the Casper dashboard at `/casper/`.

The free project-site address will be:

`https://aliphilip05.github.io/Casper-Analysis-Of-E-Galaxies-/`

The Casper dashboard will be at:

`https://aliphilip05.github.io/Casper-Analysis-Of-E-Galaxies-/casper/`

To activate Pages once the workflow is pushed, open the repository on GitHub, go to **Settings → Pages**, and choose **GitHub Actions** as the build and deployment source. Then open **Actions** and check the latest “Deploy APhlip.org to GitHub Pages” run. GitHub shows the published URL when deployment completes. Later pushes to `main` will update the site automatically.

GitHub Pages is available for public repositories on the free GitHub plan. The `.org` domain registration is separate and is not free through GitHub.

## Connect `APhlip.org` later

After registering `APhlip.org` with a domain registrar and having access to its DNS settings:

1. In your GitHub account settings, verify ownership of `APhlip.org` on the Pages settings screen. GitHub will provide a TXT record to add at your registrar.
2. In this repository, open **Settings → Pages**, enter `APhlip.org` as the custom domain, and save it before adding the website DNS records.
3. At the registrar, add these four `A` records for the apex domain (`@`): `185.199.108.153`, `185.199.109.153`, `185.199.110.153`, and `185.199.111.153`.
4. For `www.APhlip.org`, add a `CNAME` record pointing to `aliphilip05.github.io`.
5. After the DNS records propagate and GitHub enables TLS, turn on **Enforce HTTPS** in **Settings → Pages**.

DNS changes can take time to propagate. GitHub recommends verifying the domain before connecting it, and adding the domain in Pages before adding DNS records. See the [GitHub Pages custom-domain guide](https://docs.github.com/en/pages/configuring-a-custom-domain-for-your-github-pages-site/managing-a-custom-domain-for-your-github-pages-site).

## Deploying somewhere else

The personal site pages use relative links and have no build dependencies. The dashboard link in the source pages points to `../index.html`, which is correct while this directory remains inside `frontend/`. The GitHub Pages workflow adjusts that link for the combined deployment, where the dashboard is under `/casper/`.
