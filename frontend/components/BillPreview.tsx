import React from 'react';

interface BillPreviewProps {
  text: string;
}

const BillPreview: React.FC<BillPreviewProps> = ({ text }) => {
  return (
    <div className="bg-white shadow rounded-lg p-4">
      <div className="font-mono text-sm overflow-auto max-h-96 whitespace-pre-wrap">
        {text || 'No text extracted'}
      </div>
    </div>
  );
};

export default BillPreview; 