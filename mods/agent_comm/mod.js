/* ──────────────────────
   Agent‑to‑Agent Communication
   ----------------------------- */

import { div, button, text } from 'gpui';      // gpui-kit API
import { herdr } from 'herdr';                // Herdr runtime helpers

/**
 * Helper that runs an `hc` command and returns the parsed JSON.
 */
async function runHc(cmd) {
  try {
    const out = await herdr.runCommand(`hc ${cmd}`);
    return JSON.parse(out);
  } catch (e) {
    console.error('HC error', e);
    return [];
  }
}

/**
 * Send a simple string to an agent. The target agent will just `echo` the
 * message, but you can replace this with any CLI or Rustenium API call.
 */
async function sendMessage(agentId, msg) {
  await herdr.runCommand(`hc exec --agent ${agentId} "echo ${msg}"`);
}

/**
 * Main component – shows a button to refresh the list and one
 * button per agent. Clicking sends a message.
 */
export default function AgentComm() {
  const [agents, setAgents] = useState([]);

  async function refresh() {
    const list = await runHc('list agents');
    // `hc` returns a JSON array of objects like {id, name, ...}
    setAgents(list);
  }

  // Refresh automatically when the component mounts
  useEffect(() => { refresh(); }, []);

  return div(
    { className: 'agent-comm' },
    button({ onClick: refresh }, text('Refresh')),
    ...agents.map(a =>
      button({
        onClick: () => sendMessage(a.id, `Hello from ${herdr.getInfo().name}`)
      }, text(`Send to ${a.name}`)))
  );
}
