import React from 'react';

interface BillPreviewProps {
  data: {
    text: string;
    extractedData: {
      total?: number;
      date?: string;
      merchant?: string;
      items?: any[];
    };
    confidence: number;
    processingTime: number;
    source: string;
  };
}

const BillPreview: React.FC<BillPreviewProps> = ({ data }) => {
  return (
    <div className="border rounded-lg p-4 bg-gray-50">
      <div className="flex justify-between items-center mb-3">
        <h3 className="text-lg font-medium">OCR Results</h3>
        <div className="text-sm text-gray-500 flex items-center">
          <span className="mr-2">Processed by: {data.source}</span>
          <span className="bg-blue-100 text-blue-800 px-2 py-0.5 rounded-full text-xs">
            {Math.round(data.confidence * 100)}% confidence
          </span>
        </div>
      </div>
      <pre className="text-xs bg-white p-3 rounded border overflow-auto max-h-40">
        {data.text}
      </pre>
    </div>
  );
};

export default BillPreview; 