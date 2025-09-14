import "./App.css";
import { Route, Routes } from "react-router-dom";
import Login from "./pages/Login";
import Guard from "./middleware/Guard";
import Home from "./pages/Home";

export default function App() {
  return (
    <main className="bg-gray-800 text-white min-h-screen w-full h-full flex flex-col">
      <Routes>
        <Route path="/" element={<Login />} />
        <Route
          path="/home"
          element={
            <Guard>
              <Home />
            </Guard>
          }
        />
      </Routes>
    </main>
  );
}
