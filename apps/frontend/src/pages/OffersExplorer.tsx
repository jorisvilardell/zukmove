import { useEffect, useState } from 'react';
import { fetchOffers, type AggregatedOffer, applyForInternship } from '@/lib/api';
import OfferCard from '@/components/OfferCard';
import { useAuthStore } from '@/store/auth';
import { Search, AlertCircle } from 'lucide-react';

function OfferSkeleton() {
    return (
        <div className="glass-panel skeleton-card" style={{ padding: '1.5rem', display: 'flex', flexDirection: 'column', gap: '1rem' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                <div style={{ flex: 1 }}>
                    <div className="skeleton" style={{ height: '1.25rem', width: '70%', marginBottom: '0.5rem' }} />
                    <div className="skeleton" style={{ height: '0.875rem', width: '40%' }} />
                </div>
                <div className="skeleton" style={{ height: '1.5rem', width: '4rem', borderRadius: '999px' }} />
            </div>
            <div style={{ display: 'flex', gap: '1rem' }}>
                <div className="skeleton" style={{ height: '0.85rem', width: '5rem' }} />
                <div className="skeleton" style={{ height: '0.85rem', width: '10rem' }} />
            </div>
            <div style={{ padding: '1rem', background: 'rgba(0,0,0,0.2)', borderRadius: 'var(--radius-md)' }}>
                <div className="skeleton" style={{ height: '0.875rem', width: '50%', marginBottom: '0.75rem' }} />
                <div className="skeleton" style={{ height: '6px', width: '100%', marginBottom: '0.5rem' }} />
                <div className="skeleton" style={{ height: '6px', width: '100%', marginBottom: '0.5rem' }} />
                <div className="skeleton" style={{ height: '6px', width: '100%', marginBottom: '0.5rem' }} />
                <div className="skeleton" style={{ height: '6px', width: '100%' }} />
            </div>
            <div style={{ display: 'flex', gap: '0.75rem', marginTop: 'auto' }}>
                <div className="skeleton" style={{ height: '2.5rem', flex: 1, borderRadius: 'var(--radius-md)' }} />
                <div className="skeleton" style={{ height: '2.5rem', flex: 1, borderRadius: 'var(--radius-md)' }} />
            </div>
        </div>
    );
}

export default function OffersExplorer() {
    const student = useAuthStore((s) => s.student);
    const [offers, setOffers] = useState<AggregatedOffer[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const [domain, setDomain] = useState('');
    const [city, setCity] = useState('');
    const [limit, setLimit] = useState(10);

    const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);
    const [applyingId, setApplyingId] = useState<string | null>(null);

    useEffect(() => {
        loadOffers();
    }, []);

    const loadOffers = async () => {
        setLoading(true);
        setError(null);
        try {
            const data = await fetchOffers({
                domain: domain || undefined,
                city: city || undefined,
                limit: limit || undefined,
            });
            setOffers(data);
        } catch (err: any) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    const handleApply = async (offerId: string) => {
        if (!student) {
            setToast({ message: 'Please login first in the Dashboard tab.', type: 'error' });
            setTimeout(() => setToast(null), 5000);
            return;
        }

        setApplyingId(offerId);
        try {
            const result = await applyForInternship(student.id, offerId);
            setToast({
                message: `Application ${result.status}! ${result.message}`,
                type: result.status === 'Approved' ? 'success' : 'error'
            });
        } catch (err: any) {
            setToast({ message: err.message, type: 'error' });
        } finally {
            setApplyingId(null);
            setTimeout(() => setToast(null), 5000);
        }
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        loadOffers();
    };

    return (
        <div>
            <form onSubmit={handleSubmit} style={{ marginBottom: '2rem', display: 'flex', gap: '1rem', alignItems: 'flex-end', flexWrap: 'wrap' }}>
                <div style={{ flex: 1, minWidth: '160px' }}>
                    <label style={{ display: 'block', marginBottom: '0.5rem', fontSize: '0.875rem', color: 'var(--text-muted)' }}>
                        Domain
                    </label>
                    <input
                        className="input-field"
                        placeholder="e.g. IT, Life Science..."
                        value={domain}
                        onChange={(e) => setDomain(e.target.value)}
                    />
                </div>
                <div style={{ flex: 1, minWidth: '160px' }}>
                    <label style={{ display: 'block', marginBottom: '0.5rem', fontSize: '0.875rem', color: 'var(--text-muted)' }}>
                        City
                    </label>
                    <input
                        className="input-field"
                        placeholder="e.g. Paris, London..."
                        value={city}
                        onChange={(e) => setCity(e.target.value)}
                    />
                </div>
                <div style={{ minWidth: '100px' }}>
                    <label style={{ display: 'block', marginBottom: '0.5rem', fontSize: '0.875rem', color: 'var(--text-muted)' }}>
                        Limit
                    </label>
                    <input
                        type="number"
                        className="input-field"
                        min={1}
                        max={100}
                        value={limit}
                        onChange={(e) => setLimit(Number(e.target.value))}
                    />
                </div>
                <button type="submit" className="btn">
                    <Search size={18} /> Search
                </button>
            </form>

            {loading ? (
                <div className="grid-cards">
                    {Array.from({ length: 6 }).map((_, i) => (
                        <OfferSkeleton key={i} />
                    ))}
                </div>
            ) : error ? (
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem', justifyContent: 'center', color: 'var(--danger)', padding: '2rem', background: 'rgba(239, 68, 68, 0.1)', borderRadius: 'var(--radius-md)' }}>
                    <AlertCircle size={20} />
                    {error}
                </div>
            ) : offers.length === 0 ? (
                <div style={{ textAlign: 'center', padding: '4rem', color: 'var(--text-muted)', background: 'var(--bg-card)', borderRadius: 'var(--radius-lg)' }}>
                    No offers found matching your criteria.
                </div>
            ) : (
                <div className="grid-cards">
                    {offers.map((agg) => (
                        <OfferCard
                            key={agg.offer.id}
                            data={agg}
                            onApply={handleApply}
                            isApplying={applyingId === agg.offer.id}
                        />
                    ))}
                </div>
            )}

            {toast && (
                <div className={`toast ${toast.type}`}>
                    <span>{toast.message}</span>
                </div>
            )}
        </div>
    );
}
