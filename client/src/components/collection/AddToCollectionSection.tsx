"use client";

import React, { useState } from "react";
import { AddToCollectionModal } from "./AddToCollectionModal";
import { WorkType } from '@/types/work';

interface AddToCollectionSectionProps {
  targetPath: string; // e.g., "/movie/550", "/tv/1399", "/tv/1399/season/1", "/tv/1399/season/1/episode/1"
  workType: WorkType;  // "movie" | "series" | "season" | "episode"
}

export const AddToCollectionSection: React.FC<AddToCollectionSectionProps> = ({
  targetPath,
  workType,
}) => {
  const [isModalOpen, setIsModalOpen] = useState<boolean>(false);

  return (
    <>
      <div className="flex items-center space-x-3">
        {/* Trigger Button (English UI) */}
        <button
          onClick={() => setIsModalOpen(true)}
          className="px-4 py-2 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 text-white text-sm font-semibold rounded-xl transition-colors shadow-sm flex items-center space-x-2"
        >
          <span>+ Add to Collection</span>
        </button>
      </div>

      {/* Shared Collection Modal */}
      <AddToCollectionModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        targetPath={targetPath}
        workType={workType}
      />
    </>
  );
};