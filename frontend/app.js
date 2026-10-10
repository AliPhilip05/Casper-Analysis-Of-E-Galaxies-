(() => {
  const bands = ["u", "g", "r", "i", "z", "y"];
  const bandColors = { u: "#a9b4e5", g: "#9fc28a", r: "#e19f77", i: "#88bcae", z: "#a995c7", y: "#d4bd72" };
  const pageSize = 8;
  const byId = (id) => document.getElementById(id);
  const format = (number) => Number(number).toFixed(4);
  const seeded = (n) => {
    const x = Math.sin(n * 127.1 + 311.7) * 43758.5453;
    return x - Math.floor(x);
  };

  // Clearly illustrative values make the interface usable before a results file is loaded.
  const demoRows = [];
  for (let galaxy = 1; galaxy <= 103; galaxy += 1) {
    const objectId = String(410000000000000 + Math.floor(seeded(galaxy) * 890000000000000));
    const baseC = 2.15 + seeded(galaxy + 11) * 1.7;
    const baseA = 0.035 + Math.pow(seeded(galaxy + 23), 1.8) * 0.29;
    const baseS = 0.035 + seeded(galaxy + 47) * 0.21;
    bands.forEach((band, bandIndex) => {
      const phase = (bandIndex - 2.5) / 2.5;
      demoRows.push({
        index: galaxy,
        objectId,
        band,
        C: clamp(baseC + phase * 0.12 + (seeded(galaxy * 9 + bandIndex) - 0.5) * 0.13, 1.6, 4.6),
        A: clamp(baseA + phase * 0.012 + (seeded(galaxy * 13 + bandIndex + 91) - 0.5) * 0.035, 0.015, 0.48),
        S: clamp(baseS + phase * 0.01 + (seeded(galaxy * 21 + bandIndex + 17) - 0.5) * 0.03, 0.01, 0.4),
      });
    });
  }

  let rows = demoRows;
  let currentPage = 0;
  let sortKey = "index";
  let sortDirection = 1;
  let toastTimer;
  let plottedPoints = [];

  function clamp(value, min, max) { return Math.max(min, Math.min(max, value)); }
  function activeBand() { return byId("bandSelect").value; }
  function selectedRows() { return activeBand() === "all" ? rows : rows.filter((row) => row.band === activeBand()); }
  function filteredRows() {
    const query = byId("searchInput").value.trim().toLowerCase();
    const filtered = selectedRows().filter((row) => !query || String(row.objectId).toLowerCase().includes(query));
    return filtered.sort((a, b) => {
      const av = a[sortKey], bv = b[sortKey];
      const cmp = typeof av === "number" && typeof bv === "number" ? av - bv : String(av ?? "").localeCompare(String(bv ?? ""), undefined, { numeric: true });
      return cmp * sortDirection;
    });
  }

  function showToast(message) {
    const toast = byId("toast");
    toast.textContent = message;
    toast.classList.add("show");
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => toast.classList.remove("show"), 3000);
  }

  function updateSummary() {
    const uniqueObjects = new Set(rows.map((row) => row.objectId)).size;
    byId("candidateCount").textContent = String(uniqueObjects).padStart(2, "0");
    byId("measurementCount").textContent = rows.length.toLocaleString();
    const datasetName = rows === demoRows ? "Illustrative preview" : "Loaded results";
    byId("datasetState").textContent = datasetName;
    if (rows !== demoRows) byId("datasetFile").textContent = window.loadedFilename || "Local CSV · not uploaded";
    else byId("datasetFile").textContent = "Load a results CSV to explore data";
    byId("previewNote").hidden = rows !== demoRows;
  }

  function renderTable() {
    const data = filteredRows();
    const totalPages = Math.ceil(data.length / pageSize);
    currentPage = clamp(currentPage, 0, Math.max(0, totalPages - 1));
    const first = currentPage * pageSize;
    const pageRows = data.slice(first, first + pageSize);
    const body = byId("resultsBody");
    body.innerHTML = pageRows.map((row) => {
      const cWidth = clamp((row.C / 5) * 54, 4, 54);
      const aWidth = clamp((row.A / 0.5) * 44, 4, 44);
      const sWidth = clamp((row.S / 0.4) * 42, 4, 42);
      return `<tr data-object="${escapeHtml(row.objectId)}" data-band="${escapeHtml(row.band)}">
        <td class="td-index">${escapeHtml(row.index)}</td><td class="td-id">${escapeHtml(row.objectId)}</td><td><span class="band-badge" style="color:${bandColors[row.band] || "#6e8960"};border-color:${bandColors[row.band] || "#e1e9da"}">${escapeHtml(row.band)}</span></td>
        <td class="value-cell c-value">${format(row.C)}</td><td class="value-cell a-value">${format(row.A)}</td><td class="value-cell s-value">${format(row.S)}</td>
        <td class="bar-cell"><span class="mini-bars" aria-label="Relative CAS profile"><i style="width:${cWidth}px"></i><i style="width:${aWidth}px"></i><i style="width:${sWidth}px"></i></span></td></tr>`;
    }).join("");
    byId("tableEmpty").hidden = data.length > 0;
    byId("tableRange").textContent = data.length ? `Showing ${first + 1}–${Math.min(first + pageSize, data.length)} of ${data.length.toLocaleString()} measurements` : "0 measurements";
    byId("pageIndicator").textContent = `${String(totalPages ? currentPage + 1 : 0).padStart(2, "0")} / ${String(totalPages).padStart(2, "0")}`;
    byId("prevPage").disabled = currentPage <= 0;
    byId("nextPage").disabled = totalPages === 0 || currentPage >= totalPages - 1;
  }

  function chartData() {
    const groups = new Map();
    for (const row of selectedRows()) {
      if (!groups.has(row.objectId)) groups.set(row.objectId, []);
      groups.get(row.objectId).push(row);
    }
    return [...groups.values()].map((group) => {
      if (group.length === 1) return group[0];
      const average = (key) => group.reduce((sum, row) => sum + Number(row[key]), 0) / group.length;
      return { ...group[0], C: average("C"), A: average("A"), S: average("S"), band: "all" };
    });
  }

  function drawScatter() {
    const canvas = byId("scatterChart");
    const rect = canvas.getBoundingClientRect();
    if (!rect.width || !rect.height) return;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.round(rect.width * dpr);
    canvas.height = Math.round(rect.height * dpr);
    const ctx = canvas.getContext("2d");
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    const width = rect.width, height = rect.height;
    const pad = { left: 43, right: 14, top: 15, bottom: 32 };
    const plotW = width - pad.left - pad.right, plotH = height - pad.top - pad.bottom;
    const xMin = 1.5, xMax = 4.6, yMin = 0, yMax = 0.5;
    const x = (v) => pad.left + (v - xMin) / (xMax - xMin) * plotW;
    const y = (v) => pad.top + plotH - (v - yMin) / (yMax - yMin) * plotH;
    ctx.clearRect(0, 0, width, height);
    ctx.font = '9px "DM Mono", monospace';
    ctx.lineWidth = 1;
    ctx.strokeStyle = "#edf1eb";
    ctx.fillStyle = "#9aa49b";
    [0, 0.1, 0.2, 0.3, 0.4, 0.5].forEach((value) => {
      const py = y(value);
      ctx.beginPath(); ctx.moveTo(pad.left, py); ctx.lineTo(width - pad.right, py); ctx.stroke();
      ctx.textAlign = "right"; ctx.textBaseline = "middle"; ctx.fillText(value.toFixed(1), pad.left - 9, py);
    });
    [1.5, 2, 2.5, 3, 3.5, 4, 4.5].forEach((value) => {
      const px = x(value);
      ctx.beginPath(); ctx.moveTo(px, pad.top); ctx.lineTo(px, pad.top + plotH); ctx.stroke();
      ctx.textAlign = "center"; ctx.textBaseline = "top"; ctx.fillText(value.toFixed(value % 1 ? 1 : 0), px, pad.top + plotH + 8);
    });
    ctx.save(); ctx.translate(11, pad.top + plotH / 2); ctx.rotate(-Math.PI / 2); ctx.textAlign = "center"; ctx.fillStyle = "#89958b"; ctx.fillText("ASYMMETRY (A)", 0, 0); ctx.restore();
    plottedPoints = [];
    const points = chartData();
    for (const row of points) {
      const px = x(Number(row.C)), py = y(Number(row.A));
      if (!Number.isFinite(px) || !Number.isFinite(py)) continue;
      const color = row.band === "all" ? "#8eaf7a" : bandColors[row.band] || "#8eaf7a";
      ctx.beginPath(); ctx.arc(px, py, row.band === "all" ? 3.4 : 4, 0, Math.PI * 2);
      ctx.fillStyle = color; ctx.globalAlpha = 0.76; ctx.fill();
      ctx.strokeStyle = "rgba(255,255,255,.84)"; ctx.lineWidth = 1; ctx.globalAlpha = 0.9; ctx.stroke();
      plottedPoints.push({ x: px, y: py, row });
    }
    ctx.globalAlpha = 1;
    ctx.fillStyle = "#9aa49b"; ctx.font = '8px "DM Mono", monospace'; ctx.textAlign = "center"; ctx.textBaseline = "bottom";
    ctx.fillText("CONCENTRATION (C)", pad.left + plotW / 2, height - 2);
    byId("chartCount").textContent = `${points.length} OBJECTS`;
  }

  function renderProfile() {
    const width = 340, height = 236, left = 28, right = 12, top = 12, bottom = 34;
    const plotW = width - left - right, plotH = height - top - bottom;
    const byBand = bands.map((band) => {
      const group = rows.filter((row) => row.band === band);
      const med = (key) => {
        const vals = group.map((r) => Number(r[key])).filter(Number.isFinite).sort((a, b) => a - b);
        if (!vals.length) return 0;
        const mid = Math.floor(vals.length / 2);
        return vals.length % 2 ? vals[mid] : (vals[mid - 1] + vals[mid]) / 2;
      };
      return { band, C: med("C"), A: med("A"), S: med("S") };
    });
    const series = [
      { key: "C", cls: "c", min: 1.5, max: 4.8 },
      { key: "A", cls: "a", min: 0, max: 0.5 },
      { key: "S", cls: "s", min: 0, max: 0.4 },
    ];
    const x = (i) => left + i * plotW / (bands.length - 1);
    const y = (v, min, max) => top + plotH - (clamp((v - min) / (max - min), 0, 1) * plotH);
    const paths = series.map(({ key, cls, min, max }) => {
      const coords = byBand.map((entry, i) => [x(i), y(entry[key], min, max)]);
      return `<polyline class="profile-line ${cls}" points="${coords.map((p) => p.join(",")).join(" ")}" />${coords.map(([cx, cy], i) => `<circle class="profile-point ${cls}" cx="${cx}" cy="${cy}" r="3.2"><title>${bands[i]} band · median ${key}: ${format(byBand[i][key])}</title></circle>`).join("")}`;
    }).join("");
    const grid = [0, .25, .5, .75, 1].map((step) => {
      const py = top + step * plotH;
      return `<line class="profile-gridline" x1="${left}" y1="${py}" x2="${width - right}" y2="${py}" /><text class="profile-axis" x="${left - 9}" y="${py + 3}" text-anchor="end">${(1 - step).toFixed(1)}</text>`;
    }).join("");
    const labels = bands.map((band, i) => `<text class="profile-axis" x="${x(i)}" y="${height - 10}" text-anchor="middle">${band}</text>`).join("");
    byId("profileChart").innerHTML = `<svg class="profile-svg" viewBox="0 0 ${width} ${height}" role="img" aria-label="Normalized median concentration, asymmetry and smoothness by band">${grid}${paths}${labels}</svg>`;
  }

  function escapeHtml(value) {
    return String(value ?? "").replace(/[&<>"']/g, (char) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[char]);
  }

  function parseCsv(text) {
    const lines = text.replace(/^\uFEFF/, "").split(/\r?\n/).filter((line) => line.trim());
    if (lines.length < 2) throw new Error("The CSV has no measurement rows.");
    const cells = (line) => {
      const result = []; let value = "", quoted = false;
      for (let i = 0; i < line.length; i += 1) {
        const char = line[i];
        if (char === '"' && quoted && line[i + 1] === '"') { value += '"'; i += 1; }
        else if (char === '"') quoted = !quoted;
        else if (char === "," && !quoted) { result.push(value.trim()); value = ""; }
        else value += char;
      }
      result.push(value.trim());
      return result;
    };
    const header = cells(lines[0]).map((name) => name.trim().replace(/^"|"$/g, ""));
    const indexOf = (name) => header.findIndex((cell) => cell.toLowerCase() === name.toLowerCase());
    const idColumn = indexOf("objectId"), bandColumn = indexOf("band"), cColumn = indexOf("C"), aColumn = indexOf("A"), sColumn = indexOf("S"), indexColumn = indexOf("index");
    if ([idColumn, bandColumn, cColumn, aColumn, sColumn].some((i) => i < 0)) throw new Error("Expected columns: objectId, band, C, A, S.");
    const parsed = [];
    const objectIndexes = new Map();
    for (const line of lines.slice(1)) {
      const cols = cells(line);
      const objectId = cols[idColumn];
      const band = (cols[bandColumn] || "").toLowerCase();
      const C = Number(cols[cColumn]), A = Number(cols[aColumn]), S = Number(cols[sColumn]);
      if (!objectId || !bands.includes(band) || ![C, A, S].every(Number.isFinite)) continue;
      if (!objectIndexes.has(objectId)) objectIndexes.set(objectId, indexColumn >= 0 && Number.isFinite(Number(cols[indexColumn])) ? Number(cols[indexColumn]) : objectIndexes.size + 1);
      parsed.push({ index: indexColumn >= 0 && Number.isFinite(Number(cols[indexColumn])) ? Number(cols[indexColumn]) : objectIndexes.get(objectId), objectId, band, C, A, S });
    }
    if (!parsed.length) throw new Error("No valid rows found. Check that band is u, g, r, i, z, or y and C, A, S are numeric.");
    return parsed;
  }

  function downloadCsv() {
    const data = filteredRows();
    if (!data.length) { showToast("There are no visible rows to export."); return; }
    const header = ["index", "objectId", "band", "C", "A", "S"];
    const escapeCell = (value) => {
      const text = String(value ?? "");
      return /[",\n]/.test(text) ? `"${text.replace(/"/g, '""')}"` : text;
    };
    const csv = [header.join(","), ...data.map((row) => header.map((key) => escapeCell(row[key])).join(","))].join("\n");
    const url = URL.createObjectURL(new Blob([csv], { type: "text/csv;charset=utf-8" }));
    const link = document.createElement("a");
    link.href = url; link.download = "cas_results_filtered.csv"; link.click();
    URL.revokeObjectURL(url);
    showToast(`Exported ${data.length} measurement${data.length === 1 ? "" : "s"}.`);
  }

  function renderAll() {
    updateSummary();
    renderTable();
    drawScatter();
    renderProfile();
  }

  byId("bandSelect").addEventListener("change", () => { currentPage = 0; renderAll(); });
  byId("searchInput").addEventListener("input", () => { currentPage = 0; renderTable(); });
  document.querySelectorAll(".sort-button").forEach((button) => button.addEventListener("click", () => {
    const nextKey = button.dataset.sort;
    sortDirection = nextKey === sortKey ? -sortDirection : 1;
    sortKey = nextKey; currentPage = 0; renderTable();
  }));
  byId("prevPage").addEventListener("click", () => { currentPage -= 1; renderTable(); });
  byId("nextPage").addEventListener("click", () => { currentPage += 1; renderTable(); });
  byId("downloadButton").addEventListener("click", downloadCsv);
  byId("tableDownload").addEventListener("click", downloadCsv);
  byId("uploadTrigger").addEventListener("click", () => byId("csvInput").click());
  byId("csvInput").addEventListener("change", async (event) => {
    const file = event.target.files?.[0]; if (!file) return;
    try {
      const parsed = parseCsv(await file.text());
      rows = parsed; window.loadedFilename = file.name; currentPage = 0; byId("searchInput").value = ""; byId("bandSelect").value = "all"; renderAll();
      showToast(`Loaded ${parsed.length.toLocaleString()} CAS measurements from ${file.name}.`);
    } catch (error) { showToast(error.message || "Could not read that CSV file."); }
    event.target.value = "";
  });
  const overlay = byId("dropOverlay");
  let dragDepth = 0;
  window.addEventListener("dragenter", (event) => { if (event.dataTransfer?.types?.includes("Files")) { event.preventDefault(); dragDepth += 1; overlay.classList.add("visible"); } });
  window.addEventListener("dragover", (event) => { if (event.dataTransfer?.types?.includes("Files")) event.preventDefault(); });
  window.addEventListener("dragleave", (event) => { if (event.dataTransfer?.types?.includes("Files")) { dragDepth -= 1; if (dragDepth <= 0) { dragDepth = 0; overlay.classList.remove("visible"); } } });
  window.addEventListener("drop", async (event) => {
    if (!event.dataTransfer?.files?.length) return;
    event.preventDefault(); dragDepth = 0; overlay.classList.remove("visible");
    const file = event.dataTransfer.files[0];
    if (!file.name.toLowerCase().endsWith(".csv")) { showToast("Please drop a CSV results file."); return; }
    try {
      const parsed = parseCsv(await file.text());
      rows = parsed; window.loadedFilename = file.name; currentPage = 0; byId("searchInput").value = ""; byId("bandSelect").value = "all"; renderAll();
      showToast(`Loaded ${parsed.length.toLocaleString()} CAS measurements from ${file.name}.`);
    } catch (error) { showToast(error.message || "Could not read that CSV file."); }
  });
  byId("scatterChart").addEventListener("pointermove", (event) => {
    const rect = byId("scatterChart").getBoundingClientRect();
    const px = event.clientX - rect.left, py = event.clientY - rect.top;
    let closest = null, distance = 13;
    for (const point of plottedPoints) {
      const d = Math.hypot(point.x - px, point.y - py);
      if (d < distance) { closest = point; distance = d; }
    }
    const tooltip = byId("chartTooltip");
    if (!closest) { tooltip.style.display = "none"; return; }
    const row = closest.row;
    tooltip.innerHTML = `<strong>${escapeHtml(row.objectId)}</strong><br />${row.band === "all" ? "six-band mean" : `${escapeHtml(row.band)} band`} · C ${format(row.C)} · A ${format(row.A)} · S ${format(row.S)}`;
    tooltip.style.display = "block";
    tooltip.style.left = `${clamp(px + 13, 4, rect.width - tooltip.offsetWidth - 4)}px`;
    tooltip.style.top = `${clamp(py - tooltip.offsetHeight - 9, 4, rect.height - tooltip.offsetHeight - 4)}px`;
  });
  byId("scatterChart").addEventListener("pointerleave", () => { byId("chartTooltip").style.display = "none"; });
  byId("scatterChart").addEventListener("click", (event) => {
    const rect = byId("scatterChart").getBoundingClientRect();
    const px = event.clientX - rect.left, py = event.clientY - rect.top;
    const closest = plottedPoints.reduce((best, point) => {
      const dist = Math.hypot(point.x - px, point.y - py);
      return dist < best.dist ? { point, dist } : best;
    }, { point: null, dist: 13 });
    if (!closest.point) return;
    byId("searchInput").value = closest.point.row.objectId;
    byId("bandSelect").value = closest.point.row.band === "all" ? "all" : closest.point.row.band;
    currentPage = 0; renderAll();
    document.querySelector("#measurements").scrollIntoView({ behavior: "smooth", block: "start" });
  });
  byId("menuToggle").addEventListener("click", () => {
    const open = byId("mainNav").classList.toggle("open");
    byId("menuToggle").setAttribute("aria-expanded", String(open));
  });
  document.querySelectorAll(".nav-link").forEach((link) => link.addEventListener("click", () => {
    byId("mainNav").classList.remove("open"); byId("menuToggle").setAttribute("aria-expanded", "false");
    document.querySelectorAll(".nav-link").forEach((item) => item.classList.remove("active")); link.classList.add("active");
  }));
  window.addEventListener("resize", drawScatter);
  window.addEventListener("beforeprint", drawScatter);
  renderAll();
})();
