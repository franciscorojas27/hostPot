import { ReactNode, createContext, useContext, useState } from "react";
import { User } from "../types/user";
import { invoke } from '@tauri-apps/api/core';
import { useNavigate } from "react-router-dom";

interface AuthContextType {
    user: User | undefined;
    login: (user: User) => void;
    logOut: () => void;
    error: string | null;
    isAuthenticated: boolean
    checkAuth: () => Promise<boolean>;
}

type AuthProviderProps = {
    children: ReactNode;
};

export const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
    const context = useContext(AuthContext);
    if (!context) {
        throw new Error("useAuth must be used within an AuthProvider");
    }
    return context;
};

export const AuthProvider = ({ children }: AuthProviderProps) => {
    const [user, setUser] = useState<User>();
    const [error, setError] = useState<string | null>(null);
    const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);

    const navigate = useNavigate();

    const login = async (user: User) => {
        let isAuth = await invoke<boolean>('login', { user });
        if (!isAuth) {
            setError("No se encontro el usuario");
            return;
        }
        setIsAuthenticated(true)
        navigate('/home', { replace: true })
    };

    const logOut = async () => {
        await invoke('logout');
        setIsAuthenticated(false)
        setUser(undefined);
        setError(null)
    };
    const checkAuth = async () => {
        const isAuth: boolean = await invoke("is_authenticated");
        setIsAuthenticated(isAuth);
        return isAuthenticated
    }
    const value: AuthContextType = { user, login, logOut, error, checkAuth, isAuthenticated };

    return (
        <AuthContext.Provider value={value}>
            {children}
        </AuthContext.Provider>
    );
};