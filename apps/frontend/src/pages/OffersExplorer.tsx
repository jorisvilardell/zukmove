import { useEffect, useState } from 'react';
import { fetchOffers, type AggregatedOffer, applyForInternship } from '@/lib/api';
import OfferCard from '@/components/OfferCard';
import { useAuthStore } from '@/store/auth';

export default function OffersExplorer() {
    const student = useAuthStore((s) => s.student);
    const [offers, setOffers] = useState<AggregatedOffer[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    // Filters
    const [domain, setDomain] = useState('');
    const [city, setCity] = useState('');

    // Toast State
    const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);
    const [applyingId, setApplyingId] = useState<string | null>(null);

    useEffect(() => {
        loadOffers();
    }, []);

    const loadOffers = async () => {
        setLoading(true);
        setError(null);
        try {
            const data = await fetchOffers(domain || undefined, city || undefined);
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

    return (
        <div>
            <div style={{ marginBottom: '2rem', display: 'flex', gap: '1rem', alignItems: 'flex-end', flexWrap: 'wrap' }}>
                <div style={{ flex: 1, minWidth: '200px' }}>
                    <label style={{ display: 'block', marginBottom: '0.5rem', fontSize: '0.875rem', color: 'var(--text-muted)' }}>
                        Filter by Domain
                    </label>
                    <input
                        className="input-field"
                        placeholder="e.g. IT, Marketing..."
                        value={domain}
                        onChange={(e) => setDomain(e.target.value)}
                    />
                </div>
                <div style={{ flex: 1, minWidth: '200px' }}>
                    <label style={{ display: 'block', marginBottom: '0.5rem', fontSize: '0.875rem', color: 'var(--text-muted)' }}>
                        Filter by City
                    </label>
                    <input
                        className="input-field"
                        placeholder="e.g. Paris, London..."
                        value={city}
                        onChange={(e) => setCity(e.target.value)}
                    />
                </div>
                <button className="btn" onClick={loadOffers}>Search</button>
            </div>

            {loading ? (
                <div style={{ display: 'flex', justifyContent: 'center', padding: '4rem' }}>
                    <div className="loader" style={{ width: '40px', height: '40px', borderWidth: '4px' }}></div>
                </div>
            ) : error ? (
                <div style={{ color: 'var(--danger)', textAlign: 'center', padding: '2rem', background: 'rgba(239, 68, 68, 0.1)', borderRadius: 'var(--radius-md)' }}>
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
            )
            }
        </div >
    );
}
