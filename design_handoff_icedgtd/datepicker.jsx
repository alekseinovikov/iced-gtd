// IcedGTD — Calendar / date picker popover

function DatePicker({ value, onChange, onClose, anchorRect }) {
  const [view, setView] = React.useState(() => {
    const d = value ? new Date(value) : new Date();
    return { y: d.getFullYear(), m: d.getMonth() };
  });

  const today = new Date();
  const todayKey = today.toISOString().slice(0, 10);
  const fmt = (y, m, d) => `${y}-${String(m + 1).padStart(2, '0')}-${String(d).padStart(2, '0')}`;

  const monthName = new Date(view.y, view.m, 1).toLocaleDateString('en-US', { month: 'long', year: 'numeric' });
  const firstDay = new Date(view.y, view.m, 1).getDay();
  const daysInMonth = new Date(view.y, view.m + 1, 0).getDate();
  const daysInPrev = new Date(view.y, view.m, 0).getDate();

  const cells = [];
  for (let i = firstDay - 1; i >= 0; i--) cells.push({ d: daysInPrev - i, muted: true, m: view.m - 1 });
  for (let d = 1; d <= daysInMonth; d++) cells.push({ d, muted: false, m: view.m });
  while (cells.length % 7 !== 0 || cells.length < 42) cells.push({ d: cells.length - daysInMonth - firstDay + 1, muted: true, m: view.m + 1 });

  const dateAdd = (n) => { const d = new Date(); d.setDate(d.getDate() + n); return d.toISOString().slice(0, 10); };
  const nextSat = () => {
    const d = new Date();
    d.setDate(d.getDate() + ((6 - d.getDay() + 7) % 7 || 7));
    return d.toISOString().slice(0, 10);
  };

  const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

  const style = anchorRect ? {
    top: anchorRect.bottom + 6,
    left: Math.min(anchorRect.left, window.innerWidth - 320),
  } : { top: 100, left: 100 };

  return (
    <>
      <div style={{ position: 'fixed', inset: 0, zIndex: 40 }} onClick={onClose} />
      <div className="popover" style={style} onClick={e => e.stopPropagation()}>
        <div className="cal-quick">
          <button className="qbtn" onClick={() => onChange(dateAdd(0))}>
            <span className="ico">{ICONS.today}</span>
            <span>Today</span>
            <span className="day">{today.getDate()}</span>
          </button>
          <button className="qbtn" onClick={() => onChange(dateAdd(1))}>
            <span className="ico">{ICONS.clock}</span>
            <span>Tomorrow</span>
            <span className="day">{new Date(Date.now() + 864e5).toLocaleDateString('en-US', { weekday: 'short' })}</span>
          </button>
          <button className="qbtn" onClick={() => onChange(nextSat())}>
            <span className="ico">{ICONS.weekend}</span>
            <span>This weekend</span>
            <span className="day">Sat</span>
          </button>
          <button className="qbtn" onClick={() => onChange(null)}>
            <span className="ico">{ICONS.someday}</span>
            <span>Someday</span>
            <span className="day">—</span>
          </button>
        </div>

        <div className="cal-head">
          <button className="navbtn" onClick={() => setView(v => ({ y: v.m === 0 ? v.y - 1 : v.y, m: (v.m + 11) % 12 }))}>‹</button>
          <span className="month">{monthName}</span>
          <button className="navbtn" onClick={() => setView(v => ({ y: v.m === 11 ? v.y + 1 : v.y, m: (v.m + 1) % 12 }))}>›</button>
        </div>
        <div className="cal-grid">
          {dayNames.map(d => <div className="dn" key={d}>{d}</div>)}
          {cells.map((c, i) => {
            const realM = ((c.m % 12) + 12) % 12;
            const realY = view.y + Math.floor((c.m) / 12);
            const key = fmt(realY, realM, c.d);
            const isToday = key === todayKey;
            const isSelected = key === value;
            return (
              <button
                key={i}
                className={`cal-day ${c.muted ? 'muted' : ''} ${isToday ? 'today' : ''} ${isSelected ? 'selected' : ''}`}
                onClick={() => onChange(key)}
              >
                {c.d}
              </button>
            );
          })}
        </div>
        <div className="cal-foot">
          <button className="btn ghost" onClick={() => onChange(null)} style={{ height: 26, fontSize: 12 }}>Clear</button>
          <button className="btn ghost" onClick={onClose} style={{ height: 26, fontSize: 12 }}>Close</button>
        </div>
      </div>
    </>
  );
}

window.DatePicker = DatePicker;
