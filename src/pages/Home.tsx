import { FormEvent, useEffect, useState } from "react";
import { useAuth } from "../contexts/AuthContext";
import { invoke } from "@tauri-apps/api/core";

const flags: { PASSWORD_NO_CONFIGURED: string } = {
  PASSWORD_NO_CONFIGURED: "Configure antes de iniciar"
};

type HostedNetwork = {
  mode: string,
  ssid_name: string,
  max_clients: string,
  authentication: string,
  cipher: string,
  status: string,
  key: string,
};

export default function Home() {
  const [saving, setSaving] = useState<boolean>(false);
  const [autoStart, setAutoStart] = useState<boolean>(false);
  const [serverRunning, setServerRunning] = useState<boolean>(false);
  const [password, setPassword] = useState<string>();
  const [modal, setModal] = useState<{ open: boolean; message: string }>({ open: false, message: "" });
  const [state, setState] = useState<HostedNetwork>({
    mode: "",
    ssid_name: "",
    max_clients: "",
    authentication: "",
    cipher: "",
    status: "",
    key: "",
  });

  const showMessage = (message: string) => setModal({ open: true, message });
  const closeModal = () => setModal({ open: false, message: "" });

  const sanitizeKey = (raw: string | undefined) => (raw ?? "").replace(/["\\]/g, "");
  const isPasswordNotConfigured = (raw: string | undefined) => sanitizeKey(raw) === flags.PASSWORD_NO_CONFIGURED;

  useEffect(() => {
    (async () => {
      const data = await invoke<HostedNetwork>('get_hosted_network_settings_to_fronted')
      setState(data);
      setServerRunning(await invoke<boolean>('is_alive'));
      const auto = await invoke<boolean>('get_auto_start');
      setAutoStart(auto);
      if (auto) {
        const cleanedKey = sanitizeKey(data.key);
        if (isPasswordNotConfigured(cleanedKey)) {
          showMessage('Configure la contraseña antes de iniciar el servidor');
        } else {
          setServerRunning(await invoke<boolean>('start_hosted_network'));
        }
      }
    })();
  }, []);

  const handleConfigSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setSaving(true)
    let ssid = state.ssid_name
    let key = state.key
    await invoke('config_hosted_network', { ssid, key })
    setSaving(false)
  };

  const handleServerToggle = async () => {
    if (isPasswordNotConfigured(state.key)) {
      showMessage('Configure la contraseña antes de iniciar el servidor');
      return;
    }
    if (serverRunning) {
      const isRunning = await invoke<boolean>('stop_hosted_network');
      setServerRunning(!isRunning);
    } else {
      const isRunning = await invoke<boolean>('start_hosted_network');
      setServerRunning(isRunning);
    }
  };

  const handleAutoStartChange = async () => {
    await invoke('set_auto_start', { flag: !autoStart });
    setAutoStart(!autoStart);
  };

  const { logOut } = useAuth();

  return (
    <>
      <div
        className="w-full max-w-xs bg-white shadow-2xl p-5 min-w-screen space-y-6 min-h-screen"
      >
        {/* Estado de la Red */}
        <section>
          <h2 className="text-xl font-extrabold mb-3 text-blue-800 flex items-center gap-2">
            <span className="inline-block w-2 h-2 rounded-full mr-1" style={{ background: serverRunning ? "#22c55e" : "#ef4444" }} />
            Estado de la Red
          </h2>

          <div className="grid grid-cols-1 gap-2 text-gray-700 text-sm">
            <InfoRow label="SSID" value={state.ssid_name} />
            <InfoRow label="Modo" value={state.mode} />
            <InfoRow label="Clientes Máx." value={state.max_clients} />
            <InfoRow label="Autenticación" value={state.authentication} />
            <InfoRow label="Cifrado" value={state.cipher} />
            <InfoRow label="Estado" value={state.status} />
            <InfoRow label="Contraseña" value={
              <span className="tracking-widest select-all">{sanitizeKey(state.key)}</span>
            } />
          </div>
        </section>

        {/* Configurar Red */}
        <section>
          <h2 className="text-xl font-extrabold mb-3 text-blue-800">Configurar Red</h2>
          <form onSubmit={handleConfigSubmit} className="space-y-3">
            <div>
              <label className="block text-gray-600 font-semibold text-xs mb-1">SSID</label>
              <input
                type="text"
                value={state.ssid_name}
                onChange={e => setState(s => ({ ...s, ssid_name: e.target.value }))}
                className="text-black w-full border border-blue-200 rounded-lg px-3 py-2 mt-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400 bg-blue-50"
                required
                maxLength={32}
                autoComplete="off"
              />
            </div>
            <div>
              <label className="block text-gray-600 font-semibold text-xs mb-1">Contraseña</label>
              <input
                type="password"
                value={password}
                onChange={e => setPassword(e.target.value)}
                className="text-black w-full border border-blue-200 rounded-lg px-3 py-2 mt-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400 bg-blue-50"
                required
                minLength={8}
                maxLength={64}
                autoComplete="new-password"
              />
            </div>
            <button
              type="submit"
              disabled={saving}
              className={`w-full py-2 rounded-lg bg-blue-500 hover:bg-blue-700 text-white font-bold text-sm transition-colors shadow-md ${saving ? "opacity-60 cursor-not-allowed" : ""}`}
            >
              {saving ? "Guardando..." : "Guardar"}
            </button>
          </form>
        </section>

        {/* Control del Servidor */}
        <section>
          <h2 className="text-xl font-extrabold mb-3 text-blue-800">Control del Servidor</h2>
          <div className="flex flex-col space-y-3">
            <button
              onClick={handleServerToggle}
              className={`w-full py-2 rounded-lg font-bold text-sm transition-colors shadow-md flex items-center justify-center gap-2 ${serverRunning
                ? "bg-red-500 hover:bg-red-700 text-white"
                : "bg-green-500 hover:bg-green-700 text-white"
                }`}
            >
              {serverRunning ? (
                <>
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24"><rect x="6" y="6" width="12" height="12" rx="2" /></svg>
                  Detener Servidor
                </>
              ) : (
                <>
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24"><polygon points="5,3 19,12 5,21" /></svg>
                  Iniciar Servidor
                </>
              )}
            </button>
            <label className="flex items-center space-x-2 text-xs">
              <input
                type="checkbox"
                checked={autoStart}
                onChange={handleAutoStartChange}
                className="form-checkbox h-4 w-4 text-blue-600 accent-blue-600"
              />
              <span className="text-gray-700">Iniciar servidor al abrir</span>
            </label>
          </div>
        </section>

        {/* Cerrar sesión */}
        <section className="text-right">
          <button
            onClick={async () => {
              if (serverRunning) {
                const isRunning = await invoke<boolean>('stop_hosted_network');
                setServerRunning(!isRunning);
              }
              logOut()
            }}
            className="py-1.5 px-4 rounded-lg bg-gray-100 hover:bg-gray-200 text-gray-700 font-bold text-xs transition-colors shadow"
          >
            Cerrar sesión
          </button>
        </section>
      </div>
      {modal.open && (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
          <div className="absolute inset-0 bg-black/40 backdrop-blur-sm" onClick={closeModal} />
          <div className="relative bg-white rounded-xl shadow-2xl w-full max-w-sm mx-4 p-6 animate-fade-in border border-blue-100">
            <h3 className="text-lg font-bold text-blue-700 mb-2">Aviso</h3>
            <p className="text-sm text-gray-700 mb-4 whitespace-pre-line">{modal.message}</p>
            <div className="flex justify-end gap-2">
              <button
                onClick={closeModal}
                className="px-4 py-2 text-sm font-semibold rounded-lg bg-blue-600 hover:bg-blue-700 text-white shadow focus:outline-none focus:ring-2 focus:ring-blue-400"
                autoFocus
              >
                Aceptar
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}

function InfoRow({ label, value }: { label: string; value: React.ReactNode }) {
  return (
    <div className="flex items-center justify-between">
      <span className="font-semibold text-gray-600">{label}:</span>
      <span className="ml-2">{value}</span>
    </div>
  );
}
