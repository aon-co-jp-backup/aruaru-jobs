const form = document.getElementById("search-form");
const input = document.getElementById("query-input");
const resultsEl = document.getElementById("results");
const statusEl = document.getElementById("status-line");

function escapeHtml(s) {
  return String(s ?? "").replace(/[&<>"']/g, (c) => ({
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    '"': "&quot;",
    "'": "&#39;",
  })[c]);
}

function extractItems(results) {
  // aruaru-llm's /v1/search/raw response shape may vary by source adapter;
  // try a couple of plausible shapes rather than assuming one blindly.
  if (Array.isArray(results?.items)) return results.items;
  if (Array.isArray(results?.results)) return results.results;
  if (Array.isArray(results)) return results;
  return [];
}

function buildVscodeImportUrl(item) {
  const params = new URLSearchParams({
    title: item.title || "",
    url: item.url || item.link || "",
    snippet: item.snippet || item.description || "",
  });
  return `vscode://aon-co-jp.maid-cafe-programming-school/import-job?${params.toString()}`;
}

function renderResults(items) {
  resultsEl.innerHTML = "";
  if (items.length === 0) {
    resultsEl.innerHTML = "<li>該当する案件が見つかりませんでした。</li>";
    return;
  }
  for (const item of items) {
    const li = document.createElement("li");
    const title = escapeHtml(item.title || item.url || "無題の案件");
    const url = item.url || item.link || "#";
    const snippet = escapeHtml(item.snippet || item.description || "");
    const vscodeUrl = buildVscodeImportUrl(item);
    li.innerHTML = `
      <h3><a href="${escapeHtml(url)}" target="_blank" rel="noopener">${title}</a></h3>
      <p>${snippet}</p>
      <div class="job-actions">
        <a href="${escapeHtml(vscodeUrl)}">🎓 Maid Cafe Programming Schoolで開発</a>
      </div>
    `;
    resultsEl.appendChild(li);
  }
}

form.addEventListener("submit", async (e) => {
  e.preventDefault();
  const query = input.value.trim();
  if (!query) return;

  statusEl.textContent = "検索中…";
  resultsEl.innerHTML = "";

  try {
    const resp = await fetch("/api/jobs/search", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ query }),
    });
    const data = await resp.json();
    if (!resp.ok) {
      statusEl.textContent = `エラー: ${data.error || resp.status}`;
      return;
    }
    const items = extractItems(data.results);
    statusEl.textContent = `${items.length}件の案件が見つかりました。`;
    renderResults(items);
  } catch (err) {
    statusEl.textContent = `接続エラー: ${err}`;
  }
});
