// IcedGTD — Sidebar component

function Sidebar({ activeView, onSelectView, activeProject, onSelectProject, areas, counts }) {
  const [collapsed, setCollapsed] = React.useState({});
  const toggle = (id) => setCollapsed(c => ({ ...c, [id]: !c[id] }));

  const navItems = [
    { id: 'inbox', label: 'Inbox', icon: ICONS.inbox, count: counts.inbox },
    { id: 'today', label: 'Today', icon: ICONS.today, count: counts.today },
    { id: 'upcoming', label: 'Upcoming', icon: ICONS.upcoming, count: counts.upcoming },
    { id: 'anytime', label: 'Anytime', icon: ICONS.anytime, count: counts.anytime },
    { id: 'someday', label: 'Someday', icon: ICONS.someday, count: null },
  ];
  const archive = [
    { id: 'logbook', label: 'Logbook', icon: ICONS.log },
    { id: 'trash', label: 'Trash', icon: ICONS.trash },
  ];

  return (
    <aside className="sidebar">
      <div className="side-search">
        <div className="search-box">
          {ICONS.search}
          <input placeholder="Search" />
          <span className="kbd">⌘K</span>
        </div>
      </div>
      <div className="side-scroll">
        {navItems.map(item => (
          <div
            key={item.id}
            className={`nav-item ${activeView === item.id && !activeProject ? 'active' : ''}`}
            onClick={() => onSelectView(item.id)}
          >
            <span className="ico">{item.icon}</span>
            <span>{item.label}</span>
            {item.count != null && item.count > 0 && <span className="count">{item.count}</span>}
          </div>
        ))}

        <div className="side-section-label">
          Areas
          <button className="add-btn" title="New area">{ICONS.plus}</button>
        </div>

        {areas.map(area => {
          const isCollapsed = collapsed[area.id];
          return (
            <div className="area-group" key={area.id}>
              <div
                className={`area-header ${isCollapsed ? 'collapsed' : ''}`}
                onClick={() => toggle(area.id)}
              >
                <span className="chev">{ICONS.chevron}</span>
                <span className="area-dot" />
                <span>{area.name}</span>
              </div>
              {!isCollapsed && (
                <div className="proj-list">
                  {area.projects.map(p => {
                    const pct = p.tasks > 0 ? (p.done / p.tasks) * 100 : 0;
                    return (
                      <div
                        key={p.id}
                        className={`proj-item ${activeProject === p.id ? 'active' : ''}`}
                        onClick={() => onSelectProject(p.id)}
                      >
                        <span className="proj-progress" style={{ '--p': pct }} />
                        <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{p.name}</span>
                        <span className="count">{p.tasks - p.done}</span>
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
          );
        })}

        <div className="side-section-label" style={{ marginTop: 8 }}>Archive</div>
        {archive.map(item => (
          <div
            key={item.id}
            className={`nav-item ${activeView === item.id && !activeProject ? 'active' : ''}`}
            onClick={() => onSelectView(item.id)}
          >
            <span className="ico">{item.icon}</span>
            <span>{item.label}</span>
          </div>
        ))}
      </div>

      <div className="side-foot">
        <div className="avatar">SK</div>
        <div>
          <div className="name">Sara Kjartansdóttir</div>
          <div className="sub">Synced · just now</div>
        </div>
        <button className="tb-icon-btn" title="Settings">{ICONS.more}</button>
      </div>
    </aside>
  );
}

window.Sidebar = Sidebar;
