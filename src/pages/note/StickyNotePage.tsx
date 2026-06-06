import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useParams } from "react-router-dom";
import { NoteType } from "../../types/type";
import { emit, listen } from "@tauri-apps/api/event";

export default function StickyNotePage() {
  const { id } = useParams();

  const [note, setNote] = useState<NoteType | null>(null);
  const [title, setTitle] = useState("");
  const [folderId, setFolderId] = useState("");
  const [content, setContent] = useState("");

  const fetchNote = async () => {
    if (!id) return;

    await invoke("show_current_window");

    const result = await invoke<NoteType>("get_one_note", {
      id: Number(id),
    });

    setNote(result);
    setTitle(result.title);
    setFolderId(result.folder_id === null ? "" : String(result.folder_id));
    setContent(result.content);
  };

  const handleSave = async () => {
    if (!id) return;

    try {
      const result = await invoke<NoteType>("update_note_silent", {
        req: {
          id: Number(id),
          title,
          content,
          folder_id: folderId === "" ? undefined : Number(folderId),
          clear_folder: folderId === "",
        },
      });

      await emit("note-updated-sticky", {
        id: Number(id),
      });

      setNote(result);
    } catch (error) {
      console.error(error);
    }
  };

  const handleOpenManager = async () => {
    await invoke("open_manager_window");
  };

  const handleClose = async () => {
  await invoke("hide_sticky_window", {
    noteId: Number(id),
  });
};

  useEffect(() => {
    fetchNote();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [id]);

  useEffect(() => {
    const unlisten = listen<{ id: number }>("note-updated", (event) => {
      if (event.payload.id === Number(id)) {
        fetchNote();
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [id]);

  useEffect(() => {
    const prevOverflow = document.body.style.overflowY;

    document.body.style.overflowY = "hidden";

    return () => {
      document.body.style.overflowY = prevOverflow;
    };
  }, []);

  if (!note) {
    return (
      <div className="min-vh-100 bg-warning d-flex align-items-center justify-content-center">
        불러오는 중...
      </div>
    );
  }

  return (
    <div className="min-vh-100 bg-warning text-dark d-flex flex-column">
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
        />

        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          onBlur={handleSave}
          className="form-control bg-transparent border-0 flex-grow-1"
          placeholder="내용"
        />
      </main>
    </div>
  );
}
