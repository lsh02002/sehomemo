import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { NoteType } from "../../types/type";

export default function StickyHomePage() {
  useEffect(() => {
    const openStickyNotes = async () => {
      try {
        let notes = await invoke<NoteType[]>("get_pinned_notes");

        if(notes.length === 0) {
            await invoke("open_manager_window");
        }

        for (const note of notes) {
          await invoke("open_sticky_window", {
            noteId: note.id,
          });
        }

        await invoke("close_current_window");
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
