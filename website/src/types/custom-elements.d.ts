import type {DetailedHTMLProps, HTMLAttributes} from 'react';

type CustomElementProps = DetailedHTMLProps<HTMLAttributes<HTMLElement>, HTMLElement> & {
  repo?: string;
  variant?: string;
  bare?: boolean | '';
};

declare module 'react' {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace JSX {
    interface IntrinsicElements {
      'cybercore-contact': CustomElementProps;
      'cybercore-theme-chooser': CustomElementProps;
    }
  }
}
