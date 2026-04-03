import { BrowserRouter, Routes, Route, NavLink } from 'react-router-dom';
import { Compass, UserCircle, LogOut, LogIn, Settings } from 'lucide-react';
import OffersExplorer from '@/pages/OffersExplorer';
import StudentLogin from '@/pages/StudentLogin';
import StudentDashboard from '@/pages/StudentDashboard';
import PreferencesPage from '@/pages/PreferencesPage';
import { useAuthStore } from '@/store/auth';

function App() {
  const student = useAuthStore((s) => s.student);
  const logout = useAuthStore((s) => s.logout);

  return (
    <BrowserRouter>
      <div className="container">
        <header className="header">
          <div>
            <h1>Polymove</h1>
            <p>Polytech Internship Platform</p>
          </div>

          <nav className="nav-links">
            <NavLink
              to="/"
              className={({ isActive }) => `nav-link ${isActive ? 'active' : ''}`}
            >
              <Compass size={18} style={{ display: 'inline', marginRight: '6px', verticalAlign: 'middle' }} />
              Explorer
            </NavLink>

            {student ? (
              <>
                <NavLink
                  to="/dashboard"
                  className={({ isActive }) => `nav-link ${isActive ? 'active' : ''}`}
                >
                  <UserCircle size={18} style={{ display: 'inline', marginRight: '6px', verticalAlign: 'middle' }} />
                  Dashboard
                </NavLink>
                <NavLink
                  to="/preferences"
                  className={({ isActive }) => `nav-link ${isActive ? 'active' : ''}`}
                >
                  <Settings size={18} style={{ display: 'inline', marginRight: '6px', verticalAlign: 'middle' }} />
                  Preferences
                </NavLink>

                <span style={{ display: 'flex', alignItems: 'center', gap: '0.75rem', marginLeft: '1rem', color: 'var(--text-muted)', fontSize: '0.875rem' }}>
                  <span className="badge">{student.firstname} {student.name}</span>
                  <button
                    onClick={logout}
                    className="btn btn-secondary"
                    style={{ padding: '0.4rem 0.6rem', fontSize: '0.8rem' }}
                  >
                    <LogOut size={14} />
                  </button>
                </span>
              </>
            ) : (
              <NavLink
                to="/login"
                className={({ isActive }) => `nav-link ${isActive ? 'active' : ''}`}
              >
                <LogIn size={18} style={{ display: 'inline', marginRight: '6px', verticalAlign: 'middle' }} />
                Login
              </NavLink>
            )}
          </nav>
        </header>

        <main>
          <Routes>
            <Route path="/" element={<OffersExplorer />} />
            <Route path="/login" element={<StudentLogin />} />
            <Route path="/dashboard" element={<StudentDashboard />} />
            <Route path="/preferences" element={<PreferencesPage />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}

export default App;
