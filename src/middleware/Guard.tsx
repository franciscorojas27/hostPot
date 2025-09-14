import { useEffect } from "react";
import { Navigate } from "react-router-dom";
import { useAuth } from "../contexts/AuthContext";

export default function Guard({ children }: { children: React.ReactNode }) {
    const { checkAuth, isAuthenticated } = useAuth();
    useEffect(() => {
        checkAuth();
    }, [checkAuth]);

    if (!isAuthenticated) {
        return <Navigate to="/" replace />;
    }

    return children;
};