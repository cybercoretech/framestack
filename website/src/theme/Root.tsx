import {useEffect, type ReactNode} from 'react';

type RootProps = {
  children: ReactNode;
};

/**
 * Mount the shared CyberCore web component after React has hydrated.
 * This avoids asking React to hydrate DOM that the custom element rewrites.
 */
export default function Root({children}: RootProps): ReactNode {
  useEffect(() => {
    const navbarItems = document.querySelector('.navbar__items--right');
    if (!navbarItems || navbarItems.querySelector('cybercore-theme-chooser')) {
      return;
    }

    const wrapper = document.createElement('div');
    wrapper.className = 'navbar__item framestack-theme-control';

    const chooser = document.createElement('cybercore-theme-chooser');
    chooser.setAttribute('shuffle', '');
    chooser.setAttribute('hotkeys', '');
    chooser.setAttribute('compact', '');

    wrapper.append(chooser);
    navbarItems.prepend(wrapper);
  }, []);

  return <>{children}</>;
}
