// IcedGTD — Inspector / detail panel

function Inspector({ task, project, area, onChange, onClose, onOpenDate }) {
  const titleRef = React.useRef(null);
  const [title, setTitle] = React.useState(task.title);
  const [notes, setNotes] = React.useState(task.notes || '');
  const [newCheck, setNewCheck] = React.useState('');

  React.useEffect(() => { setTitle(task.title); setNotes(task.notes || ''); }, [task.id]);

  const commitTitle = () => onChange(task.id, { title: title || 'Untitled' });
  const commitNotes = () => onChange(task.id, { notes });

  const toggleChecklist = (cid) => {
    const next = (task.checklist || []).map(c => c.id === cid ? { ...c, done: !c.done } : c);
    onChange(task.id, { checklist: next });
  };
  const addChecklist = () => {
    if (!newCheck.trim()) return;
    const next = [...(task.checklist || []), { id: 'c' + Date.now(), text: newCheck.trim(), done: false }];
    onChange(task.id, { checklist: next });
    setNewCheck('');
  };

  return (
    <aside className="inspector">
      <div className="insp-header">
        <div className="crumbs">
          {area && <><span className="accent-dot" />{area.name}</>}
          {project && <><span style={{ color: 'var(--ink-4)' }}>/</span>{project.name}</>}
          {!area && !project && <><span className="accent-dot" />Inbox</>}
          <span style={{ marginLeft: 'auto' }}>
            <button className="tb-icon-btn" onClick={onClose} title="Close">{ICONS.x}</button>
          </span>
        </div>
        <h2
          contentEditable
          suppressContentEditableWarning
          ref={titleRef}
          onBlur={(e) => { setTitle(e.currentTarget.textContent); onChange(task.id, { title: e.currentTarget.textContent || 'Untitled' }); }}
          dangerouslySetInnerHTML={{ __html: task.title }}
        />
      </div>

      <div className="insp-scroll">
        <div className="field">
          <span className="label">When</span>
          <span className="value">
            <button
              className="btn ghost"
              style={{ height: 26, padding: '0 8px' }}
              onClick={(e) => onOpenDate(task.id, e.currentTarget.getBoundingClientRect(), 'when')}
            >
              {ICONS.calendar}
              <span style={{ marginLeft: 4 }}>{task.when ? fmtDate(task.when) : 'Someday'}</span>
            </button>
          </span>
        </div>
        <div className="field">
          <span className="label">Deadline</span>
          <span className={`value ${!task.deadline ? 'empty' : ''}`}>
            <button
              className="btn ghost"
              style={{ height: 26, padding: '0 8px' }}
              onClick={(e) => onOpenDate(task.id, e.currentTarget.getBoundingClientRect(), 'deadline')}
            >
              {ICONS.flag}
              <span style={{ marginLeft: 4 }}>{task.deadline ? fmtDate(task.deadline) : 'No deadline'}</span>
            </button>
          </span>
        </div>
        <div className="field">
          <span className="label">Repeat</span>
          <span className={`value ${!task.repeat ? 'empty' : ''}`}>
            {task.repeat ? <>{ICONS.repeat}<span>{task.repeat}</span></> : 'Never'}
          </span>
        </div>
        <div className="field">
          <span className="label">Project</span>
          <span className={`value ${!project ? 'empty' : ''}`}>
            {project ? <span className="proj-tag">{project.name}</span> : 'No project'}
          </span>
        </div>
        <div className="field">
          <span className="label">Tags</span>
          <span className="value">
            {(task.tags || []).map(t => <span key={t} className="tag-pill">{t}</span>)}
            <button className="chip" style={{ height: 22, padding: '0 8px' }}>{ICONS.plus}<span style={{ marginLeft: 2 }}>Tag</span></button>
          </span>
        </div>

        <div className="insp-section-title">Notes</div>
        <textarea
          className="notes"
          value={notes}
          onChange={e => setNotes(e.target.value)}
          onBlur={commitNotes}
          placeholder="Add notes…"
          rows={4}
          style={{ width: '100%', resize: 'vertical', font: 'inherit' }}
        />

        <div className="insp-section-title">Checklist</div>
        <div className="checklist-list">
          {(task.checklist || []).map(c => (
            <div key={c.id} className={`checklist-item ${c.done ? 'done' : ''}`} onClick={() => toggleChecklist(c.id)}>
              <span className={`cbx ${c.done ? 'checked' : ''}`}>{ICONS.check}</span>
              <span className="label">{c.text}</span>
            </div>
          ))}
          <div className="checklist-add">
            <span className="cbx" style={{ opacity: 0.4 }} />
            <input
              value={newCheck}
              placeholder="Add subtask…"
              onChange={e => setNewCheck(e.target.value)}
              onKeyDown={e => { if (e.key === 'Enter') addChecklist(); }}
            />
          </div>
        </div>

        <div className="insp-section-title">Activity</div>
        <div style={{ fontSize: 12, color: 'var(--ink-4)', lineHeight: 1.7 }}>
          <div>Created · 3 days ago</div>
          <div>Moved to {project?.name || 'Inbox'} · 2 days ago</div>
          {task.checklist?.some(c => c.done) && <div>Completed 1 subtask · today</div>}
        </div>
      </div>
    </aside>
  );
}

window.Inspector = Inspector;
