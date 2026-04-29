// IcedGTD — main app

const { useState, useEffect, useMemo, useRef } = React;

function App() {
  const [tweaks, setTweak] = useTweaks(/*EDITMODE-BEGIN*/{
    "theme": "light",
    "density": "cozy"
  }/*EDITMODE-END*/);

  const [tasks, setTasks] = useState(() => makeInitialTasks());
  const [view, setView] = useState('today');
  const [activeProject, setActiveProject] = useState(null);
  const [openTaskId, setOpenTaskId] = useState('t1');
  const [selection, setSelection] = useState(new Set());
  const [showInspector, setShowInspector] = useState(true);
  const [showSidebar, setShowSidebar] = useState(true);
  const [datePicker, setDatePicker] = useState(null); // {taskId, rect, field}
  const [tweaksOpen, setTweaksOpen] = useState(false);

  // Apply theme + density to root
  useEffect(() => {
    document.documentElement.setAttribute('data-theme', tweaks.theme);
    document.documentElement.setAttribute('data-density', tweaks.density);
  }, [tweaks]);

  const projectsById = useMemo(() => {
    const map = {};
    INITIAL_DATA.areas.forEach(a => a.projects.forEach(p => { map[p.id] = { ...p, areaId: a.id, areaName: a.name }; }));
    return map;
  }, []);
  const areasById = useMemo(() => Object.fromEntries(INITIAL_DATA.areas.map(a => [a.id, a])), []);

  // Counts for sidebar
  const counts = useMemo(() => {
    const today = new Date(); today.setHours(0,0,0,0);
    const isToday = (t) => t.when && new Date(t.when).setHours(0,0,0,0) <= today.getTime();
    return {
      inbox: tasks.filter(t => !t.done && !t.project && !t.when).length,
      today: tasks.filter(t => !t.done && isToday(t)).length,
      upcoming: tasks.filter(t => !t.done && t.when && new Date(t.when).setHours(0,0,0,0) > today.getTime()).length,
      anytime: tasks.filter(t => !t.done && !t.when).length,
    };
  }, [tasks]);

  // Build groups for current view
  const groups = useMemo(() => buildGroups(view, activeProject, tasks), [view, activeProject, tasks]);

  const openTask = useMemo(() => tasks.find(t => t.id === openTaskId), [tasks, openTaskId]);
  const openProject = openTask?.project ? projectsById[openTask.project] : null;
  const openArea = openProject ? areasById[openProject.areaId] : null;

  // Mutators
  const updateTask = (id, patch) => setTasks(ts => ts.map(t => t.id === id ? { ...t, ...patch } : t));
  const checkTask = (id) => updateTask(id, { done: !tasks.find(t => t.id === id).done, doneAt: new Date().toISOString().slice(0,10) });
  const checkSelection = () => { selection.forEach(id => checkTask(id)); setSelection(new Set()); };
  const scheduleSelection = (date) => { selection.forEach(id => updateTask(id, { when: date })); };
  const deleteSelection = () => { setTasks(ts => ts.filter(t => !selection.has(t.id))); setSelection(new Set()); };

  const toggleSelect = (id, range) => {
    setSelection(sel => {
      const next = new Set(sel);
      if (next.has(id)) next.delete(id); else next.add(id);
      return next;
    });
  };

  const onAddTask = (title, groupKey) => {
    const today = new Date().toISOString().slice(0,10);
    const newTask = {
      id: 't' + Date.now(),
      title,
      when: (view === 'today' || groupKey === 'today') ? today : (view === 'upcoming' ? today : null),
      project: activeProject,
      tags: [],
      notes: '',
      checklist: []
    };
    setTasks(ts => [...ts, newTask]);
    setOpenTaskId(newTask.id);
  };

  const reorder = (fromId, toId) => {
    setTasks(ts => {
      const from = ts.findIndex(t => t.id === fromId);
      const to = ts.findIndex(t => t.id === toId);
      if (from < 0 || to < 0) return ts;
      const next = [...ts];
      const [moved] = next.splice(from, 1);
      next.splice(to, 0, moved);
      return next;
    });
  };

  const headerInfo = getHeaderInfo(view, activeProject, projectsById, areasById, tasks);

  const setView2 = (v) => { setView(v); setActiveProject(null); setSelection(new Set()); };
  const setProject = (p) => { setActiveProject(p); setView('project'); setSelection(new Set()); };

  return (
    <div className="app-shell">
      <div className="window">
        <div className="titlebar">
          <div className="tb-left">
            <span className="win-btn" /> <span className="win-btn" /> <span className="win-btn" />
            <button className="tb-icon-btn" style={{ marginLeft: 6 }} onClick={() => setShowSidebar(s => !s)} title="Toggle sidebar">{ICONS.sidebar}</button>
          </div>
          <div className="tb-title"><b>IcedGTD</b> &nbsp;·&nbsp; built with Iced.rs</div>
          <div className="tb-right">
            <button className="tb-icon-btn" onClick={() => setTweak('theme', tweaks.theme === 'light' ? 'dark' : 'light')} title="Toggle theme">
              {tweaks.theme === 'light' ? ICONS.moon : ICONS.sun}
            </button>
            <button className="tb-icon-btn" onClick={() => setShowInspector(s => !s)} title="Toggle inspector">{ICONS.inspector}</button>
          </div>
        </div>

        <div className={`layout ${!showInspector || !openTask ? 'no-inspector' : ''}`} style={{ gridTemplateColumns: !showSidebar ? (showInspector && openTask ? '0 1fr 320px' : '0 1fr') : undefined }}>
          {showSidebar && (
            <Sidebar
              activeView={view}
              onSelectView={setView2}
              activeProject={activeProject}
              onSelectProject={setProject}
              areas={INITIAL_DATA.areas}
              counts={counts}
            />
          )}

          <div className="main" style={{ position: 'relative' }}>
            <div className="main-header">
              <div>
                <div className="main-title">
                  <span className="accent-dot" />
                  <h1>{headerInfo.title}</h1>
                  <span className="meta">{headerInfo.meta}</span>
                </div>
                {headerInfo.sub && <div className="main-sub">{headerInfo.sub}</div>}
              </div>
              <div className="main-actions">
                <button className="btn ghost" title="Filter">{ICONS.more}</button>
                <button className="btn primary" onClick={() => onAddTask('New task', view)}>
                  {ICONS.plus} New task <span className="kbd" style={{ background: 'oklch(from var(--accent) calc(l - 0.1) c h)', color: 'oklch(20% 0.04 220)', borderColor: 'transparent', marginLeft: 4 }}>⌘N</span>
                </button>
              </div>
            </div>

            <div className="toolbar">
              <div className="left">
                <span className="chip active">All</span>
                <span className="chip">@work</span>
                <span className="chip">@home</span>
                <span className="chip">@errand</span>
              </div>
              <div className="right">
                <span style={{ color: 'var(--ink-4)' }}>{groups.reduce((n, g) => n + g.tasks.length, 0)} tasks</span>
                <span className="kbd">⌘F</span>
              </div>
            </div>

            <TaskList
              groups={groups}
              projectsById={projectsById}
              selection={selection}
              onToggleSelect={toggleSelect}
              onCheck={checkTask}
              onOpen={(id) => { setOpenTaskId(id); setShowInspector(true); }}
              onAddTask={onAddTask}
              onReorder={reorder}
              onDateClick={(id, rect) => setDatePicker({ taskId: id, rect, field: 'when' })}
            />

            {selection.size > 0 && (
              <div className="bulk-bar">
                <span className="count-pill">{selection.size}</span>
                <span>selected</span>
                <span className="sep" />
                <button onClick={checkSelection} title="Complete">{ICONS.check}</button>
                <button onClick={(e) => setDatePicker({ taskId: '__bulk', rect: e.currentTarget.getBoundingClientRect(), field: 'when' })} title="Schedule">{ICONS.calendar}</button>
                <button title="Move to project">{ICONS.someday}</button>
                <button onClick={deleteSelection} title="Delete">{ICONS.trash}</button>
                <span className="sep" />
                <button onClick={() => setSelection(new Set())} title="Clear">{ICONS.x}</button>
              </div>
            )}
          </div>

          {showInspector && openTask && (
            <Inspector
              task={openTask}
              project={openProject}
              area={openArea}
              onChange={updateTask}
              onClose={() => setShowInspector(false)}
              onOpenDate={(id, rect, field) => setDatePicker({ taskId: id, rect, field })}
            />
          )}
        </div>

        {datePicker && (
          <DatePicker
            value={datePicker.taskId === '__bulk' ? null : tasks.find(t => t.id === datePicker.taskId)?.[datePicker.field]}
            anchorRect={datePicker.rect}
            onChange={(d) => {
              if (datePicker.taskId === '__bulk') scheduleSelection(d);
              else updateTask(datePicker.taskId, { [datePicker.field]: d });
              setDatePicker(null);
            }}
            onClose={() => setDatePicker(null)}
          />
        )}
      </div>

      <TweaksPanel title="Tweaks" defaultOpen={false}>
        <TweakSection title="Appearance">
          <TweakRadio
            label="Theme"
            value={tweaks.theme}
            options={[{ value: 'light', label: 'Light' }, { value: 'dark', label: 'Dark' }]}
            onChange={(v) => setTweak('theme', v)}
          />
          <TweakRadio
            label="Density"
            value={tweaks.density}
            options={[{ value: 'airy', label: 'Airy' }, { value: 'cozy', label: 'Cozy' }, { value: 'dense', label: 'Dense' }]}
            onChange={(v) => setTweak('density', v)}
          />
        </TweakSection>
      </TweaksPanel>
    </div>
  );
}

// ===== View → groups =====

function buildGroups(view, activeProject, tasks) {
  const today = new Date(); today.setHours(0,0,0,0);
  const todayKey = today.getTime();
  const isOnDay = (iso, off) => iso && new Date(iso).setHours(0,0,0,0) === todayKey + off * 864e5;
  const isBefore = (iso) => iso && new Date(iso).setHours(0,0,0,0) < todayKey;

  if (view === 'project' && activeProject) {
    const ts = tasks.filter(t => t.project === activeProject);
    const open = ts.filter(t => !t.done);
    const done = ts.filter(t => t.done);
    return [
      { key: 'open', title: '', tasks: open, allowAdd: true, addPlaceholder: 'Add to project…' },
      ...(done.length ? [{ key: 'done', title: 'Completed', subtitle: `${done.length}`, tasks: done }] : []),
    ];
  }
  if (view === 'today') {
    const overdue = tasks.filter(t => !t.done && isBefore(t.when));
    const morning = tasks.filter(t => !t.done && isOnDay(t.when, 0) && !t.evening);
    const evening = tasks.filter(t => !t.done && isOnDay(t.when, 0) && t.evening);
    return [
      ...(overdue.length ? [{ key: 'overdue', title: 'Overdue', subtitle: `${overdue.length}`, tasks: overdue }] : []),
      { key: 'today', title: '', tasks: morning, allowAdd: true, addPlaceholder: 'What do you want to do today?' },
      { key: 'evening', title: 'This evening', tasks: evening, allowAdd: true, addPlaceholder: 'Add for this evening…' },
    ];
  }
  if (view === 'upcoming') {
    const buckets = {};
    tasks.filter(t => !t.done && t.when && new Date(t.when).setHours(0,0,0,0) > todayKey).forEach(t => {
      const k = t.when;
      buckets[k] = buckets[k] || [];
      buckets[k].push(t);
    });
    const keys = Object.keys(buckets).sort();
    return keys.map(k => {
      const d = new Date(k);
      return {
        key: k,
        title: d.toLocaleDateString('en-US', { weekday: 'long' }),
        subtitle: d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
        tasks: buckets[k]
      };
    });
  }
  if (view === 'inbox') {
    const inbox = tasks.filter(t => !t.done && !t.project && !t.when);
    return [{ key: 'inbox', title: '', tasks: inbox, allowAdd: true, addPlaceholder: 'Capture anything on your mind…',
      empty: 'Inbox zero. Quick-capture with ⌘⇧Space.' }];
  }
  if (view === 'anytime') {
    const ts = tasks.filter(t => !t.done && !t.when);
    return [{ key: 'anytime', title: '', tasks: ts, allowAdd: true }];
  }
  if (view === 'someday') {
    return [{ key: 'someday', title: '', tasks: [], empty: 'Park ideas here you might do later.', allowAdd: true, addPlaceholder: 'Park an idea…' }];
  }
  if (view === 'logbook') {
    const ts = tasks.filter(t => t.done).sort((a,b) => (b.doneAt || '').localeCompare(a.doneAt || ''));
    return [{ key: 'log', title: 'Recently completed', tasks: ts }];
  }
  if (view === 'trash') {
    return [{ key: 'trash', title: '', tasks: [], empty: 'Trash is empty.' }];
  }
  return [{ key: 'all', title: '', tasks }];
}

function getHeaderInfo(view, activeProject, projectsById, areasById, tasks) {
  const today = new Date();
  if (view === 'project' && activeProject) {
    const p = projectsById[activeProject];
    const a = areasById[p.areaId];
    const ts = tasks.filter(t => t.project === activeProject);
    const done = ts.filter(t => t.done).length;
    return { title: p.name, meta: `${done}/${ts.length} done`, sub: `${a.name} · ${ts.length - done} open` };
  }
  switch (view) {
    case 'inbox': return { title: 'Inbox', meta: '', sub: 'Capture freely. Sort later.' };
    case 'today': return { title: 'Today', meta: today.toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric' }) };
    case 'upcoming': return { title: 'Upcoming', meta: 'Next 30 days' };
    case 'anytime': return { title: 'Anytime', meta: '', sub: 'Tasks without a date — pick when you have a moment.' };
    case 'someday': return { title: 'Someday', meta: '', sub: 'Maybe later. Maybe never.' };
    case 'logbook': return { title: 'Logbook', meta: 'All-time' };
    case 'trash': return { title: 'Trash', meta: '' };
    default: return { title: 'IcedGTD', meta: '' };
  }
}

ReactDOM.createRoot(document.getElementById('root')).render(<App />);
