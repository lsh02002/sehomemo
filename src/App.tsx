import React from "react";
import { HashRouter, Route, Routes } from "react-router-dom";
import NewNotePage from "./pages/note/NewNotePage";
import NoteListPage from "./pages/note/NoteListPage";
import UpdateNotePage from "./pages/note/UpdateNotePage";

function App() {
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<NoteListPage />} />
        <Route path="/create" element={<NewNotePage />} />
        <Route path="/update/:id" element={<UpdateNotePage />} />
      </Routes>
    </HashRouter>
  );
}

export default App;
