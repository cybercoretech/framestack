import {useEffect, type ReactNode} from 'react';
import {useLocation} from '@docusaurus/router';

type RootProps = {
  children: ReactNode;
};

/**
 * Mounts the shared Cybercore theme chooser in the navbar after React has
 * hydrated, so React never hydrates DOM that the custom element rewrites.
 *
 * The MASTER DOCS link is added the same way.
 *
 * Owner rules: the chooser sits in the bar on every screen width and never
 * inside the hamburger sidebar. Docusaurus hides `.navbar__item` elements below
 * 997px and moves them into the sidebar, so the wrapper deliberately does not
 * use that class. It is re-checked after every route change.
 */
export default function Root({children}: RootProps): ReactNode {
  const location = useLocation();

  useEffect(() => {
    const right = document.querySelector('.navbar__items--right');
    if (!right) {
      return;
    }
    // MASTER DOCS: the family-wide link to the Cybercore docs, kept in the bar too.
    if (!right.querySelector('[data-cybercore-master-docs]')) {
      const docs = document.createElement('a');
      docs.className = 'master-docs';
      docs.href = 'https://cybercoretech.github.io/cybercore/docs.html';
      docs.dataset.cybercoreMasterDocs = '';
      docs.textContent = 'MASTER DOCS ↗';
      right.prepend(docs);
    }
    if (right.querySelector('cybercore-theme-chooser')) {
      return;
    }
    const wrapper = document.createElement('div');
    wrapper.className = 'framestack-theme-control';
    const chooser = document.createElement('cybercore-theme-chooser');
    chooser.setAttribute('shuffle', '');
    chooser.setAttribute('hotkeys', '');
    chooser.setAttribute('compact', '');
    chooser.setAttribute('match', '.navbar__items--right .navbar__link');
    wrapper.append(chooser);
    right.append(wrapper);
  }, [location.pathname]);

  return <>{children}</>;
}
