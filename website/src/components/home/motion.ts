import {useEffect, useRef, useState} from 'react';

/** True when the visitor asked for reduced motion (false during server rendering). */
export function useReducedMotion(): boolean {
  const [reduce, setReduce] = useState(false);
  useEffect(() => {
    const query = window.matchMedia('(prefers-reduced-motion: reduce)');
    setReduce(query.matches);
    const update = () => setReduce(query.matches);
    query.addEventListener('change', update);
    return () => query.removeEventListener('change', update);
  }, []);
  return reduce;
}

const GLYPHS = '01<>/{}[]=;#$_';

/**
 * Resolves the element's text from scrambled glyphs the first time it scrolls
 * into view. The real text is in the server-rendered HTML; this only animates it.
 */
export function useScramble<T extends HTMLElement>(): React.RefObject<T | null> {
  const ref = useRef<T>(null);
  useEffect(() => {
    const el = ref.current;
    if (!el || window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
    const targets = Array.from(el.querySelectorAll<HTMLElement>('[data-text]'));
    if (targets.length === 0) return;
    let frame = 0;
    const observer = new IntersectionObserver(([entry]) => {
      if (!entry?.isIntersecting) return;
      observer.disconnect();
      const start = performance.now();
      const tick = (now: number) => {
        let done = true;
        for (const t of targets) {
          const text = t.dataset.text ?? '';
          const revealed = Math.floor((now - start) / 28);
          if (revealed < text.length) done = false;
          t.textContent = Array.from(text, (ch, i) =>
            i < revealed || ch === ' ' ? ch : GLYPHS[Math.floor(Math.random() * GLYPHS.length)],
          ).join('');
        }
        if (!done) frame = requestAnimationFrame(tick);
      };
      frame = requestAnimationFrame(tick);
    });
    observer.observe(el);
    return () => {
      observer.disconnect();
      cancelAnimationFrame(frame);
      for (const t of targets) t.textContent = t.dataset.text ?? '';
    };
  }, []);
  return ref;
}

/** Adds `is-in` to elements with `data-reveal` as they enter the viewport. */
export function useReveal(): void {
  useEffect(() => {
    const els = Array.from(document.querySelectorAll<HTMLElement>('[data-reveal]'));
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
      els.forEach((el) => el.classList.add('is-in'));
      return;
    }
    const observer = new IntersectionObserver(
      (entries) => entries.forEach((e) => {
        if (e.isIntersecting) {
          e.target.classList.add('is-in');
          observer.unobserve(e.target);
        }
      }),
      {rootMargin: '0px 0px -8% 0px'},
    );
    els.forEach((el) => observer.observe(el));
    return () => observer.disconnect();
  }, []);
}
