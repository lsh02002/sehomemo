import React from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import NewNotePage from "./pages/note/NewNotePage";
import NoteListPage from "./pages/note/NoteListPage";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<NoteListPage />} />
        <Route path="/create" element={<NewNotePage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
