import React, { useState } from 'react';
import { AlertCircle } from 'lucide-react';
import { DashboardLayout } from '../components/layout/DashboardLayout';

/**
 * Renders a single page to:
 * 1) Create a new proxy session (hostname + optional IP).
 * 2) Display and manage a list of generated sessions.
 */
export function ProxySessions() {
  const [hostname, setHostname] = useState('');
  const [ip, setIp] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [sessions, setSessions] = useState<Array<{ token: string; url: string }>>([]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    if (!hostname) {
      setError('Hostname is required.');
      return;
    }

    try {
      const response = await fetch('https://api.directdns.net/api/create', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          hostname,
          ip_address: ip || null,
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to create session');
      }

      const data = await response.json();

      // Construct the URL
      const newSession = {
        token: data.token,
        url: `https://${data.token}.directdns.net/`,
        name: hostname,
        ip: ip
      };

      // Prepend new session to the top of the list
      setSessions((prev) => [newSession, ...prev]);

      // Clear form inputs
      setHostname('');
      setIp('');
    } catch (err: any) {
      setError(err.message);
    }
  };

  const handleCopy = async (url: string) => {
    try {
      await navigator.clipboard.writeText(url);
      alert('Copied!');
    } catch (err) {
      alert('Failed to copy!');
    }
  };

  return (
    <DashboardLayout>
      {/* Form to create proxy sessions */}
      <div className="bg-white shadow-lg rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-medium text-gray-900">
            Create New Proxy Session
          </h2>
          <p className="mt-1 text-sm text-gray-500">
            Enter the hostname and optional IP address to create a new proxy session. Please note that the proxy link will not be active forever, and will be removed after a certain period of time (usually a few days, can be sooner or later as the project is still in alpha).
          </p>
        </div>

        <div className="px-6 py-6">
          <form onSubmit={handleSubmit} className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700">
                Hostname <span className="text-red-500">*</span>
              </label>
              <div className="mt-1">
                <input
                  type="text"
                  className="w-full border border-gray-300 rounded-md shadow-sm px-4 py-2 focus:ring-blue-500 focus:border-blue-500"
                  placeholder="example.com"
                  value={hostname}
                  onChange={(e) => setHostname(e.target.value)}
                />
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700">
                IP Address (Optional)
              </label>
              <div className="mt-1">
                <input
                  type="text"
                  className="w-full border border-gray-300 rounded-md shadow-sm px-4 py-2 focus:ring-blue-500 focus:border-blue-500"
                  placeholder="45.8.22.46"
                  value={ip}
                  onChange={(e) => setIp(e.target.value)}
                />
              </div>
            </div>

            {error && (
              <div className="rounded-md bg-red-50 p-4">
                <div className="flex">
                  <AlertCircle className="h-5 w-5 text-red-400" />
                  <div className="ml-3">
                    <p className="text-sm text-red-700">{error}</p>
                  </div>
                </div>
              </div>
            )}

            <div className="flex justify-end">
              <button
                type="submit"
                className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
              >
                Create Session
              </button>
            </div>
          </form>
        </div>
      </div>

      {/* List of created sessions */}
      {sessions.length > 0 && (
        <div className="mt-8 bg-white shadow-lg rounded-lg p-6">
          <h3 className="text-md font-medium text-gray-900 mb-4">
            Your Proxy Sessions (Resets on page reload)
          </h3>
          <ul className="space-y-4">
            {sessions.map((session, idx) => (
              <li
                key={idx}
                className="flex flex-col sm:flex-row sm:items-center sm:justify-between bg-gray-50 rounded-lg p-4"
              >
                <span className="font-medium text-gray-700 break-all">
                  {session.url}
                </span>
                <span className="font-normal text-gray-700 break-all">
                  {session.name} ({session.ip || 'No IP'})
                </span>
                <div className="mt-2 sm:mt-0 sm:ml-4 flex space-x-2">
                  <button
                    onClick={() => handleCopy(session.url)}
                    className="bg-blue-600 text-white px-3 py-2 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                  >
                    Copy
                  </button>
                  <a
                    href={session.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="bg-green-600 text-white px-3 py-2 rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500"
                  >
                    Open
                  </a>
                </div>
              </li>
            ))}
          </ul>
        </div>
      )}
    </DashboardLayout>
  );
}
