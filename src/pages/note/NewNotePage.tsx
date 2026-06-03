import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function NewNotePage() {
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSave = async () => {
    if (!title.trim()) {
      alert("제목을 입력해주세요.");
      return;
    }

    try {
      setLoading(true);

      await invoke("create_note", {
        req: {
          title,
          content,
        },
      });

      alert("메모 저장 완료");

      setTitle("");
      setContent("");
    } catch (error) {
      console.error(error);
      alert("메모 저장 실패");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-dark text-light min-vh-100 d-flex flex-column">
      {/* Header */}
      <header className="border-bottom border-secondary px-4 py-3">
        <h1 className="h3 m-0">새 메모</h1>
      </header>

      {/* Main */}
      <main className="container-fluid flex-grow-1 py-4">
        <div className="d-flex flex-column h-100 gap-3">
          {/* Title */}
          <input
            type="text"
            placeholder="제목 입력"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="form-control form-control-lg bg-dark text-light border-secondary"
          />

          {/* Content */}
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

          {/* Footer */}
          <div className="d-flex justify-content-end">
            <button
              onClick={handleSave}
              disabled={loading}
              className="btn btn-primary px-4"
            >
              {loading ? "저장 중..." : "저장"}
            </button>
          </div>
        </div>
      </main>
    </div>
  );
}
