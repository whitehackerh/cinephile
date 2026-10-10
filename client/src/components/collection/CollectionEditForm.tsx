'use client';

import { FormEvent, useEffect } from 'react';

type Props = {
  title: string;
  description: string;
  isSaving: boolean;
  onTitleChange: (value: string) => void;
  onDescriptionChange: (value: string) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  onClose: () => void;
};

export default function CollectionEditForm({
  title,
  description,
  isSaving,
  onTitleChange,
  onDescriptionChange,
  onSubmit,
  onClose
}: Props) {
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && !isSaving) {
        onClose();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isSaving, onClose]);

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center overflow-y-auto bg-black/80 p-4 backdrop-blur-sm"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget && !isSaving) {
          onClose();
        }
      }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-labelledby="collection-edit-title"
        className="my-auto w-full max-w-xl border border-white/15 bg-zinc-950 p-5 shadow-2xl sm:p-8"
      >
        <div className="mb-6 flex items-center justify-between gap-4">
          <h2 id="collection-edit-title" className="font-serif text-2xl text-white">
            Edit collection
          </h2>
          <button
            type="button"
            onClick={onClose}
            disabled={isSaving}
            aria-label="Close edit modal"
            className="inline-flex h-9 w-9 shrink-0 items-center justify-center border border-white/15 text-gray-400 transition-colors hover:border-white/40 hover:text-white disabled:opacity-50"
          >
            <svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M18 6 6 18" />
              <path d="m6 6 12 12" />
            </svg>
          </button>
        </div>
        <form onSubmit={onSubmit} className="space-y-5">
          <div>
            <label htmlFor="collection-title" className="mb-2 block text-xs font-semibold uppercase tracking-widest text-gray-400">
              Title
            </label>
            <input
              id="collection-title"
              value={title}
              onChange={(event) => onTitleChange(event.target.value)}
              maxLength={100}
              required
              className="w-full border border-white/15 bg-black/60 px-4 py-3 text-sm text-white outline-none transition-colors focus:border-gold"
            />
          </div>
          <div>
            <label htmlFor="collection-description" className="mb-2 block text-xs font-semibold uppercase tracking-widest text-gray-400">
              Description
            </label>
            <textarea
              id="collection-description"
              value={description}
              onChange={(event) => onDescriptionChange(event.target.value)}
              rows={4}
              className="w-full resize-y border border-white/15 bg-black/60 px-4 py-3 text-sm text-white outline-none transition-colors focus:border-gold"
            />
          </div>
          <div className="flex justify-end gap-3 border-t border-white/10 pt-5">
            <button
              type="button"
              onClick={onClose}
              disabled={isSaving}
              className="border border-white/15 px-5 py-3 text-xs font-bold uppercase tracking-[0.2em] text-gray-300 transition-colors hover:border-white/40 hover:text-white disabled:opacity-50"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={isSaving}
              className="bg-gold px-5 py-3 text-xs font-bold uppercase tracking-[0.2em] text-black transition-opacity hover:opacity-80 disabled:opacity-50"
            >
              {isSaving ? 'Saving...' : 'Save changes'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}