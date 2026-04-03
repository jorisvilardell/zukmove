import { BrowserRouter, Routes, Route, NavLink } from 'react-router-dom';
import { Compass, UserCircle } from 'lucide-react';
import OffersExplorer from '@/pages/OffersExplorer';
import StudentDashboard from '@/pages/StudentDashboard';

function App() {
  return (
    <BrowserRouter>
      <div className="container">
        <header className="header">
          <div>
            <h1>Zukmove Gateway</h1>
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
            <NavLink
              to="/dashboard"
              className={({ isActive }) => `nav-link ${isActive ? 'active' : ''}`}
            >
              <UserCircle size={18} style={{ display: 'inline', marginRight: '6px', verticalAlign: 'middle' }} />
              Student Dashboard
            </NavLink>
          </nav>
        </header>

        <main>
          <Routes>
            <Route path="/" element={<OffersExplorer />} />
            <Route path="/dashboard" element={<StudentDashboard />} />
          </Routes>
        </main>
      </div >
    </BrowserRouter >
  );
}

export default App;
