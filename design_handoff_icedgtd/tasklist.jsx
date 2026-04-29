// IcedGTD — Task list view + task row + inline composer

function fmtDate(iso) {
  if (!iso) return null;
  const d = new Date(iso);
  const today = new Date();
  today.setHours(0,0,0,0);
  const target = new Date(d); target.setHours(0,0,0,0);
  const diff = Math.round((target - today) / 864e5);
  if (diff === 0) return 'Today';
  if (diff === 1) return 'Tomorrow';
  if (diff === -1) return 'Yesterday';
  if (diff < 0) return `${Math.abs(diff)}d overdue`;
  if (diff < 7) return d.toLocaleDateString('en-US', { weekday: 'short' });
  return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
}

function dueClass(iso) {
  if (!iso) return '';
  const today = new Date(); today.setHours(0,0,0,0);
  const target = new Date(iso); target.setHours(0,0,0,0);
  const diff = (target - today) / 864e5;
  if (diff < 0) return 'overdue';
  if (diff === 0) return 'today';
  return 'scheduled';
}

function TaskRow({ task, projectName, selected, onToggleSelect, onCheck, onOpen, onDateClick, isDragging, dropAbove, draggable, onDragStart, onDragOver, onDrop, onDragEnd }) {
  const [bursting, setBursting] = React.useState(false);
  const handleCheck = (e) => {
    e.stopPropagation();
    if (!task.done) {
      setBursting(true);
      setTimeout(() => setBursting(false), 500);
    }
    onCheck(task.id);
  };

  const checklistTotal = task.checklist?.length || 0;
  const checklistDone = task.checklist?.filter(c => c.done).length || 0;

  return (
    <div
      className={`task-row ${selected ? 'selected' : ''} ${task.done ? 'done' : ''} ${isDragging ? 'dragging' : ''} ${dropAbove ? 'drop-above' : ''}`}
      onClick={(e) => {
        if (e.metaKey || e.ctrlKey || e.shiftKey) onToggleSelect(task.id, e.shiftKey);
        else onOpen(task.id);
      }}
      draggable={draggable}
      onDragStart={onDragStart}
      onDragOver={onDragOver}
      onDrop={onDrop}
      onDragEnd={onDragEnd}
    >
      <span className="grip" onMouseDown={e => e.stopPropagation()}>{ICONS.grip}</span>
      <span
        className={`cbx ${task.done ? 'checked' : ''} ${task.deadline && !task.done ? 'deadline' : ''} ${bursting ? 'bursting' : ''}`}
        onClick={handleCheck}
      >
        {ICONS.check}
      </span>
      <div className="task-title">
        <span className="label">{task.title}</span>
        {task.notes && <span className="notes-dot" title="Has notes" />}
        {checklistTotal > 0 && (
          <span className="checklist-mini">
            <svg width="9" height="9" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3"><polyline points="20 6 9 17 4 12"/></svg>
            {checklistDone}/{checklistTotal}
          </span>
        )}
      </div>
      <div className="task-meta">
        {task.repeat && (
          <span className="repeat" title={task.repeat}>{ICONS.repeat}</span>
        )}
        {task.tags?.slice(0, 1).map(t => (
          <span key={t} className="tag-pill">{t}</span>
        ))}
        {projectName && <span className="proj-tag">{projectName}</span>}
        {task.when && !task.done && (
          <span
            className={`due ${dueClass(task.when)}`}
            onClick={(e) => { e.stopPropagation(); onDateClick(task.id, e.currentTarget.getBoundingClientRect()); }}
          >
            {ICONS.calendar} {fmtDate(task.when)}
          </span>
        )}
        {task.done && task.doneAt && (
          <span className="due scheduled">{fmtDate(task.doneAt)}</span>
        )}
      </div>
      <span></span>
    </div>
  );
}

function NewTaskInline({ onAdd, placeholder = 'Add a task…' }) {
  const [focused, setFocused] = React.useState(false);
  const [value, setValue] = React.useState('');
  const inputRef = React.useRef(null);

  const submit = () => {
    if (value.trim()) {
      onAdd(value.trim());
      setValue('');
      setTimeout(() => inputRef.current?.focus(), 10);
    }
  };

  return (
    <div className={`new-task ${focused ? 'focused' : ''}`} onClick={() => inputRef.current?.focus()}>
      <span></span>
      <span className="cbx" style={{ opacity: 0.5 }} />
      <input
        ref={inputRef}
        value={value}
        placeholder={placeholder}
        onChange={e => setValue(e.target.value)}
        onFocus={() => setFocused(true)}
        onBlur={() => setFocused(false)}
        onKeyDown={(e) => {
          if (e.key === 'Enter') submit();
          if (e.key === 'Escape') { setValue(''); inputRef.current?.blur(); }
        }}
      />
      {focused && (
        <span className="new-meta">
          <span className="kbd">↵</span> save · <span className="kbd">esc</span> cancel
        </span>
      )}
    </div>
  );
}

function TaskList({ groups, projectsById, selection, onToggleSelect, onCheck, onOpen, onAddTask, onReorder, onDateClick }) {
  const [dragId, setDragId] = React.useState(null);
  const [dropTargetId, setDropTargetId] = React.useState(null);

  return (
    <div className="task-scroll">
      {groups.map((group, gi) => (
        <div key={group.key}>
          {group.title && (
            <div className="section-divider">
              {group.title}
              {group.subtitle && <span style={{ color: 'var(--ink-3)', textTransform: 'none', letterSpacing: 0 }}>{group.subtitle}</span>}
            </div>
          )}
          {group.tasks.length === 0 && group.empty && (
            <div className="empty-hint" style={{ padding: '20px 12px' }}>{group.empty}</div>
          )}
          {group.tasks.map(task => (
            <TaskRow
              key={task.id}
              task={task}
              projectName={task.project ? projectsById[task.project]?.name : null}
              selected={selection.has(task.id)}
              onToggleSelect={onToggleSelect}
              onCheck={onCheck}
              onOpen={onOpen}
              onDateClick={onDateClick}
              draggable={true}
              isDragging={dragId === task.id}
              dropAbove={dropTargetId === task.id && dragId !== task.id}
              onDragStart={() => setDragId(task.id)}
              onDragOver={(e) => { e.preventDefault(); setDropTargetId(task.id); }}
              onDrop={(e) => { e.preventDefault(); if (dragId && dragId !== task.id) onReorder(dragId, task.id); setDropTargetId(null); }}
              onDragEnd={() => { setDragId(null); setDropTargetId(null); }}
            />
          ))}
          {group.allowAdd && <NewTaskInline onAdd={(t) => onAddTask(t, group.key)} placeholder={group.addPlaceholder || 'Add a task…'} />}
        </div>
      ))}
    </div>
  );
}

window.TaskList = TaskList;
window.fmtDate = fmtDate;
window.dueClass = dueClass;
