import { useState } from 'react';
import { fetchStudent, fetchStudentsByDomain, createStudent, type Student } from '@/lib/api';
import { useAuthStore } from '@/store/auth';
import { UserCircle, UserPlus, LogIn, Search, Users } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

export default function StudentLogin() {
    const login = useAuthStore((s) => s.login);
    const navigate = useNavigate();

    const [loginId, setLoginId] = useState('');
    const [loginLoading, setLoginLoading] = useState(false);

    const [showCreateForm, setShowCreateForm] = useState(false);
    const [firstname, setFirstname] = useState('');
    const [name, setName] = useState('');
    const [domain, setDomain] = useState('');
    const [creating, setCreating] = useState(false);

    const [searchDomain, setSearchDomain] = useState('');
    const [students, setStudents] = useState<Student[]>([]);
    const [searchLoading, setSearchLoading] = useState(false);
    const [hasSearched, setHasSearched] = useState(false);

    const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);

    const showToast = (message: string, type: 'success' | 'error') => {
        setToast({ message, type });
        setTimeout(() => setToast(null), 5000);
    };

    const doLogin = (student: Student) => {
        login(student);
        showToast(`Welcome, ${student.firstname}!`, 'success');
        setTimeout(() => navigate('/dashboard'), 300);
    };

    const handleLogin = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!loginId.trim()) return;

        setLoginLoading(true);
        try {
            const studentData = await fetchStudent(loginId.trim());
            doLogin(studentData);
        } catch {
            showToast('Student not found. Check your ID or create an account.', 'error');
        } finally {
            setLoginLoading(false);
        }
    };

    const handleCreate = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!firstname.trim() || !name.trim() || !domain.trim()) return;

        setCreating(true);
        try {
            const created = await createStudent({ firstname: firstname.trim(), name: name.trim(), domain: domain.trim() });
            showToast(`Account created! Your ID: ${created.id}`, 'success');
            doLogin(created);
        } catch (err: any) {
            showToast(err.message, 'error');
        } finally {
            setCreating(false);
        }
    };

    const handleSearch = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!searchDomain.trim()) return;

        setSearchLoading(true);
        setHasSearched(true);
        try {
            const results = await fetchStudentsByDomain(searchDomain.trim());
            setStudents(results);
        } catch (err: any) {
            showToast(err.message, 'error');
            setStudents([]);
        } finally {
            setSearchLoading(false);
        }
    };

    const handleSelectStudent = (student: Student) => {
        doLogin(student);
    };

    return (
        <div style={{ maxWidth: '600px', margin: '0 auto' }}>
            {/* Login by ID */}
            <div className="glass-panel" style={{ padding: '2rem', textAlign: 'center', marginBottom: '1.5rem' }}>
                <UserCircle size={48} color="var(--primary)" style={{ marginBottom: '1rem' }} />
                <h2>Student Login</h2>
                <p style={{ marginBottom: '1.5rem' }}>Enter your Student ID or select from the list below.</p>

                <form onSubmit={handleLogin} style={{ display: 'flex', gap: '1rem' }}>
                    <input
                        type="text"
                        className="input-field"
                        placeholder="Your Student ID (UUID)"
                        value={loginId}
                        onChange={(e) => setLoginId(e.target.value)}
                        style={{ flex: 1, marginBottom: 0 }}
                        autoFocus
                    />
                    <button type="submit" className="btn" disabled={!loginId.trim() || loginLoading}>
                        {loginLoading ? <div className="loader" /> : <><LogIn size={18} /> Login</>}
                    </button>
                </form>
            </div>

            {/* Student List by Domain */}
            <div className="glass-panel" style={{ padding: '2rem', marginBottom: '1.5rem' }}>
                <h3 style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginBottom: '1rem' }}>
                    <Users size={20} color="var(--primary)" /> Find Students
                </h3>
                <form onSubmit={handleSearch} style={{ display: 'flex', gap: '1rem', marginBottom: '1rem' }}>
                    <input
                        type="text"
                        className="input-field"
                        placeholder="Search by domain (e.g. IT, Life Science...)"
                        value={searchDomain}
                        onChange={(e) => setSearchDomain(e.target.value)}
                        style={{ flex: 1, marginBottom: 0 }}
                    />
                    <button type="submit" className="btn btn-secondary" disabled={!searchDomain.trim() || searchLoading}>
                        {searchLoading ? <div className="loader" /> : <><Search size={18} /></>}
                    </button>
                </form>

                {searchLoading ? (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
                        {Array.from({ length: 3 }).map((_, i) => (
                            <div key={i} className="skeleton" style={{ height: '3.5rem', borderRadius: 'var(--radius-md)' }} />
                        ))}
                    </div>
                ) : hasSearched && students.length === 0 ? (
                    <p style={{ textAlign: 'center', fontSize: '0.875rem' }}>No students found for this domain.</p>
                ) : students.length > 0 ? (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
                        {students.map((s) => (
                            <button
                                key={s.id}
                                onClick={() => handleSelectStudent(s)}
                                className="student-list-item"
                            >
                                <div>
                                    <span style={{ fontWeight: 600, color: 'var(--text-main)' }}>
                                        {s.firstname} {s.name}
                                    </span>
                                    <span className="badge" style={{ marginLeft: '0.5rem' }}>{s.domain}</span>
                                </div>
                                <span style={{ fontSize: '0.7rem', color: 'var(--text-muted)', fontFamily: 'monospace' }}>
                                    {s.id.slice(0, 8)}...
                                </span>
                            </button>
                        ))}
                    </div>
                ) : null}
            </div>

            {/* Create Account */}
            <div className="glass-panel" style={{ padding: '2rem', textAlign: 'center' }}>
                <button
                    type="button"
                    className="btn btn-secondary"
                    onClick={() => setShowCreateForm(!showCreateForm)}
                    style={{ marginBottom: showCreateForm ? '1.5rem' : 0 }}
                >
                    <UserPlus size={18} />
                    {showCreateForm ? 'Cancel' : 'Create New Account'}
                </button>

                {showCreateForm && (
                    <form onSubmit={handleCreate} style={{ textAlign: 'left' }}>
                        <div className="input-group">
                            <label style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>First Name</label>
                            <input type="text" className="input-field" placeholder="Jean" value={firstname} onChange={(e) => setFirstname(e.target.value)} />
                        </div>
                        <div className="input-group">
                            <label style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>Last Name</label>
                            <input type="text" className="input-field" placeholder="Dupont" value={name} onChange={(e) => setName(e.target.value)} />
                        </div>
                        <div className="input-group">
                            <label style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>Domain</label>
                            <input type="text" className="input-field" placeholder="e.g. IT, Life Science" value={domain} onChange={(e) => setDomain(e.target.value)} />
                        </div>
                        <button type="submit" className="btn btn-success" disabled={creating || !firstname.trim() || !name.trim() || !domain.trim()} style={{ width: '100%' }}>
                            {creating ? <div className="loader" /> : 'Create & Login'}
                        </button>
                    </form>
                )}
            </div>

            {toast && (
                <div className={`toast ${toast.type}`}><span>{toast.message}</span></div>
            )}
        </div>
    );
}
