"use client";

import { useRouter } from "next/navigation";
import React from "react";

export default function AddRule() {
  const [rule, setRule] = React.useState("");
  const [rules, setRules] = React.useState<string[]>([]);
  const handleAddRule = () => {
    setRule("asd");
    setRules([...rules, rule]);
  };
  const router = useRouter();

  return (
    <div className="flex flex-grow flex-col self-start p-4">
      <button
        onClick={() => router.back()}
        className="self-start mb-4 text-charcoalGray font-semibold"
      >
        ← Back
      </button>

      <div className="flex flex-col items-center justify-center">
        <h1 className="text-3xl mb-6 font-[var(--font-roboto)] font-light">
          Add New Rule
        </h1>

        <div className="flex-col">
          <form className="flex items-end gap-4 mb-2">
            <div className="w-full lg:w-2/5 mb-2">
              <label
                htmlFor="rule"
                className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
              >
                Rule name
              </label>
              <input
                type="text"
                id="rule"
                className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                placeholder="Allow SSH"
                required
              />
            </div>

            <div className="w-full lg:w-1/5 mb-2">
              <label
                htmlFor="protocol"
                className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
              >
                Protocol
              </label>
              <select
                id="protocol"
                className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
              >
                <option>TCP</option>
                <option>UDP</option>
                <option>TCP/UDP</option>
              </select>
            </div>

            <div className="w-full lg:w-1/6 mb-2">
              <label
                htmlFor="port"
                className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
              >
                Port number
              </label>
              <input
                type="text"
                id="port"
                className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                placeholder="22"
                required
              />
            </div>

            <div className="w-full lg:w-1/5 mb-2">
              <label
                htmlFor="action"
                className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
              >
                Action
              </label>
              <select
                id="action"
                className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
              >
                <option>ALLOW</option>
                <option>BLOCK</option>
                <option>MONITOR</option>
              </select>
            </div>
          </form>

          <div className="flex justify-center">
            <button
              onClick={handleAddRule}
              className="bg-green-600 text-white px-16 py-2 rounded-lg hover:bg-green-700"
            >
              Add Rule
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
