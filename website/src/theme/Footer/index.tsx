import type {ReactNode} from 'react';
import Footer from '@theme-original/Footer';
import type FooterType from '@theme/Footer';
import type {WrapperProps} from '@docusaurus/types';

type Props = WrapperProps<typeof FooterType>;

/**
 * The universal Cybercore contact band (kit/contact.js renders it from the org
 * site's data/contact.json), above Docusaurus's own footer. The link inside is
 * the no-JS fallback.
 */
export default function FooterWrapper(props: Props): ReactNode {
  return (
    <>
      <section className="cc-contact-band" aria-label="Contact Cybercore Tech">
        <cybercore-contact repo="framestack">
          <a href="mailto:dev@cybercoretech.net">dev@cybercoretech.net</a>
        </cybercore-contact>
      </section>
      <Footer {...props} />
    </>
  );
}
