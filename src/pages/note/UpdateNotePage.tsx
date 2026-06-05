import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate, useParams } from "react-router-dom";
import { BackwardButton } from "../../components/BackwardButton";
import SelectInput, { Option } from "../../components/SelectInput";
import { FolderType, NoteType } from "../../types/type";

export default function UpdateNotePage() {
  const navigate = useNavigate();
  const { id } = useParams();

  const [title, setTitle] = useState("");
  const [folderId, setFolderId] = useState("");
  const [folderOptions, setFolderOptions] = useState<Option[]>([]);
  const [content, setContent] = useState("");
  const [loading, setLoading] = useState(false);

  const fetchFolders = async () => {
    try {
      setLoading(true);

      const result = await invoke<FolderType[]>("get_folders");
      setFolderOptions(
        result?.map((folder) => ({
          label: folder.name,
          value: String(folder.id),
        })),
      );
    } catch (error) {
      console.error(error);
      alert("메모 목록을 불러오지 못했습니다.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchFolders();
  }, []);

  useEffect(() => {
    const loadNote = async () => {
      try {
        const note = await invoke<NoteType>("get_one_note", {
          id: Number(id),
        });

        setTitle(note.title);
        setFolderId(String(note.folder_id));
        setContent(note.content);
      } catch (error) {
        console.error(error);
        alert("메모 불러오기 실패");
        navigate("/");
      }
    };

    loadNote();
  }, [id, navigate]);

  const handleUpdate = async () => {
    if (!title.trim()) {
      alert("제목을 입력해주세요.");
      return;
    }

    try {
      setLoading(true);

      await invoke("update_note", {
        req: {
          id: Number(id),
          title,
          folder_id: folderId === "" ? null : Number(folderId),
          content,
        },
      });

      navigate("/");
    } catch (error) {
      console.error(error);
      alert(error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-dark text-light min-vh-100 d-flex flex-column">
      <BackwardButton />
      <header className="border-bottom border-secondary px-4 py-3">
        <h1 className="h3 m-0">메모 수정</h1>
      </header>

      <main className="container-fluid flex-grow-1 py-4">
        <div className="d-flex flex-column h-100 gap-3">
          <input
            type="text"
            placeholder="제목 입력"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="form-control form-control-lg bg-dark text-light border-secondary"
          />

          <SelectInput
            name="folder"
            title="폴더"
            value={folderId}
            setValue={setFolderId}
            options={folderOptions}
          />

          <textarea
            placeholder="메모를 입력하세요..."
            value={content}
            onChange={(e) => setContent(e.target.value)}
            className="form-control bg-dark text-light border-secondary flex-grow-1"
            style={{
              minHeight: "400px",
              resize: "none",
            }}
          />

          <div className="d-flex justify-content-end">
            <button
              type="button"
              onClick={handleUpdate}
              disabled={loading}
              className="btn btn-primary px-4"
            >
              {loading ? "수정 중..." : "수정"}
            </button>
          </div>
        </div>
      </main>
    </div>
  );
}
