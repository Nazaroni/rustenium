import { div, stack, text } from 'gpui';
import { useState, useEffect } from 'gpui';
import { herdr } from 'herdr';

/* Helper: Run an `hc` command and parse the JSON output */
async function fetchAgents() {
  const out = await herdr.runCommand('hc list agents');
  try {
    return JSON.parse(out);
  } catch (e) {
    console.error('Failed to parse agent list:', e);
    return [];
  }
}

/* Helper: Format a duration in milliseconds into HH:MM:SS */
function formatDuration(ms) {
  const totalSeconds = Math.floor(ms / 1000);
  const hrs = Math.floor(totalSeconds / 3600);
  const mins = Math.floor((totalSeconds % 3600) / 60);
  const secs = totalSeconds % 60;
  return `${hrs > 0 ? `${hrs}:` : ''}${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
}

/* Main component – shows cards grouped by status and a working timer */
export default function AgentsCards() {
  const [agents, setAgents] = useState([]);
  const [now, setNow] = useState(Date.now()); // current time for timers

  async function load() {
    const data = await fetchAgents();
    setAgents(data || []);
  }

  /* Refresh the agent list once on mount and every minute to keep status up‑to‑date. */
  useEffect(() => {
    load();
    const interval = setInterval(load, 60000); // refresh every 60s
    return () => clearInterval(interval);
  }, []);

  /* Update the clock tick once per second so timers stay accurate. */
  useEffect(() => {
    const timer = setInterval(() => setNow(Date.now()), 1000);
    return () => clearInterval(timer);
  }, []);

  /* Group agents by status – the `status` field is whatever your
     agent prints with `hc list agents`.  Adjust the keys if they differ. */
  const groups = {
    'needs you': [],
    working: [],
    done: []
  };

  for (const a of agents) {
    if (a.status === 'needs you') groups['needs you'].push(a);
    else if (a.status === 'working') groups.working.push(a);
    else if (a.status === 'done') groups.done.push(a);
  }

  return div(
    { className: 'agents-columns', style: { display: 'flex', gap: '1rem' } },
    ...Object.entries(groups).map(([title, list]) =>
      stack({ direction: 'vertical', gap: '0.5rem' }, [
        text(title),
        ...list.map(a => {
          // Compute elapsed time if a start timestamp is present.
          let elapsedMs = 0;
          if (a.start_time) {
            const ts = typeof a.start_time === 'number'
              ? a.start_time
              : Date.parse(a.start_time);
            if (!isNaN(ts)) {
              elapsedMs = now - ts;
              if (elapsedMs < 0) elapsedMs = 0; // safety
            }
          }
          return div(
            {
              className: 'agent-card',
              style: { border: '1px solid #ccc', padding: '0.5rem', borderRadius: '4px' }
            },
            text(`${a.name} (${a.id})`),
            text(`Status: ${a.status}`),
            a.status === 'working' && elapsedMs > 0
              ? text(`Working for ${formatDuration(elapsedMs)}`)
              : null
          );
        })
      ])
    )
  );
}
