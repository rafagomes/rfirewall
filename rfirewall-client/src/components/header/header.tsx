"use client";

import Link from "next/link";
import { useState } from "react";

export default function Header() {
  const [menuOpen, setMenuOpen] = useState(false);
  const [isRunning, setIsRunning] = useState(false);

  return (
    <header className="bg-mutedSlateGray h-16 w-full flex items-center justify-between p-4">
      {/* Left Menu */}
      <div className="flex space-x-4">
        <Link href="/add-rule">
          <button
            type="button"
            className="bg-softCoral hover:bg-softCoralHover text-white px-4 py-2 rounded-lg"
          >
            Add Rule
          </button>
        </Link>
        <button
          type="button"
          className={`px-4 py-2 rounded-lg text-white font-semibold ${
            isRunning
              ? "bg-vibrantSlate hover:bg-royalBlue"
              : "bg-mutedTeal hover:bg-mutedTealHover"
          }`}
          onClick={() => setIsRunning(!isRunning)}
        >
          {isRunning ? "Stop" : "Start"}
        </button>
      </div>

      {/* Right Hamburger Menu */}
      <div className="relative">
        <button className="text-softCoral focus:outline-none">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            className="h-8 w-8"
            onClick={() => setMenuOpen(!menuOpen)}
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
            />
          </svg>
        </button>

        {menuOpen && (
          <div className="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg z-10">
            <Link
              href="/"
              className="block px-4 py-2 text-gray-800 hover:bg-gray-100"
            >
              Home
            </Link>
            <Link
              href="/add-rule"
              className="block px-4 py-2 text-gray-800 hover:bg-gray-100"
            >
              Add rules
            </Link>
          </div>
        )}
      </div>
    </header>
  );
}
