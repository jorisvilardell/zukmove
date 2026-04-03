export interface Offer {
    id: string;
    name: string;
    company: string;
    city: string;
    domain: string;
    url: string;
}

export interface CityScore {
    city: string;
    country: string;
    updated_at: string;
    quality_of_life: number;
    safety: number;
    economy: number;
    culture: number;
}

export interface News {
    id: string;
    name: string;
    source: string;
    date: string;
    tags: string[];
    city: string;
    country: string;
}

export interface AggregatedOffer {
    offer: Offer;
    city_score: CityScore | null;
    latest_news: News[] | null;
}

export interface CreateStudentRequest {
    firstname: string;
    name: string;
    domain: string;
}

export interface Student {
    id: string;
    firstname: string;
    name: string;
    domain: string;
}

export interface Internship {
    id: string;
    student_id: string;
    offer_id: string;
    status: 'Approved' | 'Rejected';
    message: string;
}

const API_BASE = 'http://localhost:8080';

export async function fetchOffers(domain?: string, city?: string): Promise<AggregatedOffer[]> {
    const params = new URLSearchParams();
    if (domain) params.append('domain', domain);
    if (city) params.append('city', city);

    const url = `${API_BASE}/offer${params.toString() ? '?' + params.toString() : ''}`;

    const res = await fetch(url);
    if (!res.ok) throw new Error('Failed to fetch offers');
    return res.json();
}

export async function fetchStudent(id: string): Promise<Student> {
    const res = await fetch(`${API_BASE}/student/${id}`);
    if (!res.ok) throw new Error('Student not found');
    return res.json();
}

export async function createStudent(data: CreateStudentRequest): Promise<Student> {
    const res = await fetch(`${API_BASE}/student`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data)
    });

    const result = await res.json();
    if (!res.ok) throw new Error(result.error || 'Failed to create student');
    return result;
}

export async function fetchRecommendedOffers(studentId: string): Promise<AggregatedOffer[]> {
    const res = await fetch(`${API_BASE}/student/${studentId}/recommended-offers`);
    if (!res.ok) throw new Error('Failed to fetch recommended offers');
    return res.json();
}

export async function applyForInternship(studentId: string, offerId: string): Promise<Internship> {
    const res = await fetch(`${API_BASE}/internship`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ student_id: studentId, offer_id: offerId })
    });

    const data = await res.json();
    if (!res.ok) throw new Error(data.error || 'Application failed');
    return data;
}

export type SortByScore = 'quality_of_life' | 'safety' | 'economy' | 'culture';

export function sortOffersByScore(offers: AggregatedOffer[], sortBy: SortByScore): AggregatedOffer[] {
    return [...offers].sort((a, b) => {
        const scoreA = a.city_score?.[sortBy] ?? 0;
        const scoreB = b.city_score?.[sortBy] ?? 0;
        return scoreB - scoreA;
    });
}
