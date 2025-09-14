import { useAuth } from "../contexts/AuthContext";
import { useState } from "react";
import { User } from "../types/user";
import { z } from "zod";

const userSchema = z.object({
    name: z.string().min(1, "El nombre es requerido"),
    password: z.string().min(1, "La contraseña es requerida"),
}); 

export default function Login() {
    const { login, error } = useAuth();
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [validationErrors, setValidationErrors] = useState<{ name?: string; password?: string }>({});

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        const user: User = {
            name: username.trim(),
            password: password,
        };
        const result = userSchema.safeParse(user);
        if (!result.success) {
            const fieldErrors: { name?: string; password?: string } = {};
            result.error.issues.forEach((err) => {
                if (err.path[0] === "name") fieldErrors.name = err.message;
                if (err.path[0] === "password") fieldErrors.password = err.message;
            });
            setValidationErrors(fieldErrors);
            return;
        }
        setValidationErrors({});
        login(user);
    };

    return (
        <div className="flex items-center justify-center min-h-screen bg-gradient-to-br from-gray-800 to-gray-900">
            <div className="bg-white/10 backdrop-blur-md rounded-xl shadow-lg p-8 w-full max-w-sm border border-gray-700">
                <h2 className="text-3xl font-bold text-center text-white mb-6">Iniciar sesión</h2>
                <form
                    onSubmit={handleSubmit}
                    className="flex flex-col gap-4"
                >
                    <div>
                        <label className="block text-gray-200 mb-1" htmlFor="username">
                            Usuario
                        </label>
                        <input
                            id="username"
                            type="text"
                            className="w-full px-4 py-2 rounded-lg border border-gray-600 bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-white"
                            placeholder="Tu usuario"
                            value={username}
                            onChange={(e) => setUsername(e.target.value)}
                            autoComplete="username"
                        />
                        {validationErrors.name && (
                            <p className="text-red-400 text-xs mt-1">{validationErrors.name}</p>
                        )}
                    </div>
                    <div>
                        <label className="block text-gray-200 mb-1" htmlFor="password">
                            Contraseña
                        </label>
                        <input
                            id="password"
                            type="password"
                            className="w-full px-4 py-2 rounded-lg border border-gray-600 bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-white"
                            placeholder="Tu contraseña"
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                            autoComplete="current-password"
                        />
                        {validationErrors.password && (
                            <p className="text-red-400 text-xs mt-1">{validationErrors.password}</p>
                        )}
                    </div>
                    {error && (
                        <p className="text-red-400 text-sm text-center">{error}</p>
                    )}
                    <button
                        type="submit"
                        className="w-full py-2 mt-2 rounded-lg bg-blue-500 hover:bg-blue-700 text-white font-semibold transition-colors"
                    >
                        Iniciar sesión
                    </button>
                </form>
            </div>
        </div>
    );
}