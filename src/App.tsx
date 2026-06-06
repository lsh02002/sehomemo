import React from "react";
import { HashRouter, Route, Routes } from "react-router-dom";
import NewNotePage from "./pages/note/NewNotePage";
import NoteListPage from "./pages/note/NoteListPage";
import UpdateNotePage from "./pages/note/UpdateNotePage";
import FolderViewPage from "./pages/folder/FolderViewPage";
import NewFolderPage from "./pages/folder/NewFolderPage";
import TrashPage from "./pages/note/TrashPage";
import { BootstrapToastContainer } from "./components/Toast";
import StickyNotePage from "./pages/note/StickyNotePage";
import StickyHomePage from "./pages/note/StickyHomePage";
import NewStickyNotePage from "./pages/note/NewStickyNotePage";

function App() {
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<StickyHomePage />} />
        <Route path="/sticky/new" element={<NewStickyNotePage />} />
        <Route path="/sticky/:id" element={<StickyNotePage />} />
        <Route path="/manager" element={<NoteListPage />} />
        <Route path="/create" element={<NewNotePage />} />
        <Route path="/update/:id" element={<UpdateNotePage />} />
        <Route path="/folder/create" element={<NewFolderPage />} />
        <Route path="/folder/:folderId" element={<FolderViewPage />} />
        <Route path="/trash" element={<TrashPage />} />
      </Routes>
      <BootstrapToastContainer />
    </HashRouter>
  );
}

export default App;
