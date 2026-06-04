import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate, useParams } from "react-router-dom";
import { FolderType, NoteType } from "../../types/type";

export default function FolderViewPage() {
  const navigate = useNavigate();
  const { folderId } = useParams();

  const [folder, setFolder] = useState<FolderType | null>(null);
  const [notes, setNotes] = useState<NoteType[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      if (!folderId) return;

      try {
        setLoading(true);

        const [folderResult, notesResult] = await Promise.all([
          invoke<FolderType>("get_folder", {
            id: Number(folderId),
          }),
          invoke<NoteType[]>("get_notes_by_folder", {
            folderId: Number(folderId),
          }),
        ]);

        setFolder(folderResult);
        setNotes(notesResult);
      } catch (error) {
        console.error(error);
        alert("폴더를 불러오지 못했습니다.");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [folderId]);

  return (
    <div className="min-vh-100 bg-dark text-white d-flex flex-column">
      <header className="d-flex align-items-center justify-content-between border-bottom border-secondary px-4 py-3">
        <div>
          <button
            onClick={() => navigate(-1)}
            className="btn btn-outline-secondary btn-sm mb-2"
          >
            뒤로가기
          </button>

          <h1 className="h4 fw-bold mb-0">{folder?.name ?? "폴더"}</h1>
        </div>

        <button
          onClick={() => navigate(`/create?folderId=${folderId}`)}
          className="btn btn-primary"
        >
          새 메모
        </button>
      </header>

      <main className="flex-grow-1 overflow-auto p-4">
        {loading ? (
          <p className="text-secondary">불러오는 중...</p>
        ) : notes.length === 0 ? (
          <div className="d-flex h-100 align-items-center justify-content-center text-secondary">
            이 폴더에 메모가 없습니다.
          </div>
        ) : (
          <div className="row g-3">
            {notes.map((note) => (
              <div key={note.id} className="col-12">
                <div className="card bg-black text-white border-secondary">
                  <div className="card-body">
                    <button
                      onClick={() => navigate(`/update/${note.id}`)}
                      className="btn text-start text-white w-100 p-0 border-0"
                    >
                      <h2 className="h5 fw-semibold mb-2">
                        {note.title || "제목 없음"}
                      </h2>

                      <p className="text-secondary small mb-2 text-truncate">
                        {note.content || "내용 없음"}
                      </p>

                      <p className="text-secondary small mb-0">
                        수정일: {new Date(note.updated_at).toLocaleString()}
                      </p>
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}
