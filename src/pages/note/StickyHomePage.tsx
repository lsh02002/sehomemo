import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { NoteType } from "../../types/type";

export default function StickyHomePage() {
  useEffect(() => {
    const openStickyNotes = async () => {
      try {
        let notes = await invoke<NoteType[]>("get_pinned_notes");

        const pinnedNotes = notes.filter((note) => note.is_pinned);

        if (pinnedNotes.length === 0) {
          await invoke("open_manager_window");
        } else {
          await Promise.all(
            notes.map((note) =>
              invoke("preload_sticky_window", {
                noteId: note.id,
              }),
            ),
          );

          await Promise.all(
            pinnedNotes.map((note) =>
              invoke("show_sticky_window", {
                noteId: note.id,
              }),
            ),
          );
        }

      } catch (error) {
        console.error(error);
        alert("포스트잇을 열지 못했습니다.");
      }
    };

    openStickyNotes();
  }, []);

  return (
    <div className="min-vh-100 bg-dark text-white d-flex align-items-center justify-content-center">
      포스트잇을 여는 중...
    </div>
  );
}
