// IcedGTD — sample data + helpers

const ICONS = {
  inbox: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M22 12h-6l-2 3h-4l-2-3H2"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11Z"/></svg>,
  today: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 2"/></svg>,
  upcoming: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="3" y="4" width="18" height="18" rx="2"/><path d="M16 2v4M8 2v4M3 10h18"/></svg>,
  anytime: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M3 12h18M3 6h18M3 18h12"/></svg>,
  someday: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M21 15a4 4 0 0 1-4 4H8l-5 3V6a4 4 0 0 1 4-4h10a4 4 0 0 1 4 4z"/></svg>,
  log: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><path d="M12 15V3"/></svg>,
  trash: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/><path d="M10 11v6M14 11v6"/></svg>,
  chevron: <svg className="feather chev" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5"><polyline points="6 9 12 15 18 9"/></svg>,
  plus: <svg className="feather" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.2"><path d="M12 5v14M5 12h14"/></svg>,
  search: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="11" cy="11" r="7"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>,
  check: <svg className="feather" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3.5"><polyline points="20 6 9 17 4 12"/></svg>,
  dash: <svg className="feather" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3.5"><line x1="5" y1="12" x2="19" y2="12"/></svg>,
  flag: <svg className="feather" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M4 22V4M4 4h13l-2 4 2 4H4"/></svg>,
  repeat: <svg className="feather" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="17 1 21 5 17 9"/><path d="M3 11V9a4 4 0 0 1 4-4h14"/><polyline points="7 23 3 19 7 15"/><path d="M21 13v2a4 4 0 0 1-4 4H3"/></svg>,
  grip: <svg width="10" height="14" viewBox="0 0 10 14" fill="currentColor"><circle cx="2" cy="2" r="1.2"/><circle cx="2" cy="7" r="1.2"/><circle cx="2" cy="12" r="1.2"/><circle cx="8" cy="2" r="1.2"/><circle cx="8" cy="7" r="1.2"/><circle cx="8" cy="12" r="1.2"/></svg>,
  sun: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"/></svg>,
  moon: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>,
  sidebar: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="9" y1="3" x2="9" y2="21"/></svg>,
  inspector: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="15" y1="3" x2="15" y2="21"/></svg>,
  calendar: <svg className="feather" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="3" y="4" width="18" height="18" rx="2"/><path d="M16 2v4M8 2v4M3 10h18"/></svg>,
  clock: <svg className="feather" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 2"/></svg>,
  weekend: <svg className="feather" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="6" cy="14" r="3"/><circle cx="18" cy="14" r="3"/><path d="M6 11V8a3 3 0 0 1 6 0v3M12 11V8a3 3 0 0 1 6 0v3"/></svg>,
  archive: <svg className="feather" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="21 8 21 21 3 21 3 8"/><rect x="1" y="3" width="22" height="5"/><line x1="10" y1="12" x2="14" y2="12"/></svg>,
  more: <svg className="feather" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="1.2"/><circle cx="19" cy="12" r="1.2"/><circle cx="5" cy="12" r="1.2"/></svg>,
  x: <svg className="feather" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>,
};

// Initial app data — areas → projects → tasks
const INITIAL_DATA = {
  areas: [
    {
      id: 'a-personal', name: 'Personal',
      projects: [
        { id: 'p-trip', name: 'Iceland trip', emoji: '🧊', tasks: 7, done: 3 },
        { id: 'p-home', name: 'Apartment refresh', emoji: '🪴', tasks: 4, done: 1 },
        { id: 'p-read', name: 'Reading list', emoji: '📚', tasks: 12, done: 8 },
      ],
    },
    {
      id: 'a-work', name: 'Work',
      projects: [
        { id: 'p-launch', name: 'Q3 product launch', emoji: '🚀', tasks: 18, done: 5 },
        { id: 'p-hire', name: 'Hire designer', emoji: '🧑\u200d🎨', tasks: 6, done: 2 },
      ],
    },
    {
      id: 'a-side', name: 'Side projects',
      projects: [
        { id: 'p-iced', name: 'IcedGTD', emoji: '❄️', tasks: 9, done: 0 },
      ],
    },
  ],
};

// Tasks. due/scheduled stored as ISO string or null. project = id.
function makeInitialTasks() {
  const today = new Date();
  const iso = (d) => d.toISOString().slice(0, 10);
  const addDays = (n) => { const d = new Date(today); d.setDate(d.getDate() + n); return iso(d); };
  return [
    // Today
    { id: 't1', title: 'Review Q3 launch checklist with Maya', when: addDays(0), deadline: addDays(2), project: 'p-launch', tags: ['@work'], notes: 'Bring the customer-research deck. Confirm legal review owner before EOD.', checklist: [{ id: 'c1', text: 'Pull last week\u2019s metrics', done: true }, { id: 'c2', text: 'Print agenda', done: false }, { id: 'c3', text: 'Confirm room booking', done: false }] },
    { id: 't2', title: 'Book flights to Reykjavík', when: addDays(0), deadline: addDays(7), project: 'p-trip', tags: ['@home'], notes: '', checklist: [] },
    { id: 't3', title: 'Daily Spanish — 15 min', when: addDays(0), repeat: 'Every day', project: null, tags: ['@home'], notes: '', checklist: [] },
    { id: 't4', title: 'Reply to Jonas about contract draft', when: addDays(0), project: 'p-hire', tags: ['@work'], notes: '', checklist: [] },
    { id: 't5', title: 'Pick up dry cleaning', when: addDays(0), project: null, tags: ['@errand'], notes: '', checklist: [] },
    // This evening
    { id: 't6', title: 'Cook dinner — pasta with the new pan', when: addDays(0), evening: true, project: null, tags: ['@home'], notes: '', checklist: [] },
    { id: 't7', title: 'Call Mom', when: addDays(0), evening: true, project: null, tags: ['@home'], notes: '', checklist: [] },
    // Upcoming / scheduled
    { id: 't8', title: 'Submit visa application', when: addDays(3), deadline: addDays(5), project: 'p-trip', tags: ['@home'], notes: '' },
    { id: 't9', title: 'Quarterly review prep', when: addDays(5), project: 'p-launch', tags: ['@work'], notes: '' },
    { id: 't10', title: 'Order replacement chair', when: addDays(2), project: 'p-home', tags: ['@home'], notes: '' },
    { id: 't11', title: 'Weekly review', when: addDays(2), repeat: 'Every Friday', project: null, tags: [], notes: '' },
    // Anytime / no date
    { id: 't12', title: 'Audit the bookshelf', when: null, project: 'p-home', tags: ['@home'], notes: '' },
    { id: 't13', title: 'Sketch landing page hero', when: null, project: 'p-iced', tags: ['@work'], notes: '' },
    { id: 't14', title: 'Set up keyboard shortcuts in Iced', when: null, project: 'p-iced', tags: ['@work'], notes: 'See iced::keyboard module' },
    { id: 't15', title: 'Pack list — start draft', when: null, project: 'p-trip', tags: [], notes: '' },
    // Completed
    { id: 't16', title: 'Buy thermal layers', done: true, doneAt: addDays(-1), project: 'p-trip', tags: ['@home'] },
    { id: 't17', title: 'Renew passport', done: true, doneAt: addDays(-3), project: 'p-trip', tags: [] },
  ];
}

window.ICONS = ICONS;
window.INITIAL_DATA = INITIAL_DATA;
window.makeInitialTasks = makeInitialTasks;
