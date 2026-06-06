import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";
import { NoteType } from "../../types/type";

export default function NewStickyNotePage() {
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");

  useEffect(() => {
    const prevOverflow = document.body.style.overflowY;

    document.body.style.overflowY = "hidden";

    return () => {
      document.body.style.overflowY = prevOverflow;
    };
  }, []);

  useEffect(() => {
    invoke("show_current_window");
  }, []);

  const handleOpenManager = async () => {
    await invoke("open_manager_window");
  };

  const handleClose = async () => {
    await invoke("close_current_window");
  };

  const handleSave = async () => {
    if (!title.trim() && !content.trim()) return;

    try {
      const note = await invoke<NoteType>("create_note", {
        req: {
          title: title.trim() || "제목 없음",
          content,
          folder_id: null,
        },
      });

      await invoke("update_note", {
        req: {
          id: note.id,
          is_pinned: true,
        },
      });

      await invoke("open_sticky_window", {
        noteId: note.id,
      });

      await emit("note-created", {
        id: note.id,
      });

      await invoke("close_current_window");
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div
      className="bg-warning text-dark d-flex flex-column"
      style={{
        width: "100vw",
        height: "100vh",
        overflow: "hidden",
      }}
    >
      <header
        data-tauri-drag-region
        className="d-flex align-items-center justify-content-between px-2 py-1 border-bottom border-dark"
        style={{ cursor: "grab" }}
      >
        <button onClick={handleOpenManager} className="btn btn-sm btn-dark">
          관리
        </button>

        <button onClick={handleClose} className="btn btn-sm btn-outline-dark">
          ×
        </button>
      </header>

      <main className="flex-grow-1 d-flex flex-column p-2">
        <input
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          onBlur={handleSave}
          className="form-control form-control-sm bg-transparent border-0 fw-bold mb-2"
          placeholder="제목"
          autoFocus
        />

        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          onBlur={handleSave}
          className="form-control bg-transparent border-0 flex-grow-1"
          placeholder="내용"
          style={{
            resize: "none",
            overflowY: "auto",
          }}
        />
      </main>
    </div>
  );
}
