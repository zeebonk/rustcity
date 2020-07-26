use super::utils::clamp;

pub enum ZoomDirection {
    In,
    Out,
}

pub struct Camera {
    pub x: f64,
    pub y: f64,
    _zoom: f64,
    _step: f64,
}

impl Camera {
    const MIN_ZOOM: f64 = 0.125;
    const MAX_ZOOM: f64 = 8.;
    const ZOOM_STEPS: f64 = 101.;

    pub fn new() -> Camera {
        Camera {
            x: 0.,
            y: 0.,
            _zoom: 1.,
            _step: 51.,
        }
    }

    pub fn zoom_at(&mut self, x: f64, y: f64, zoom: ZoomDirection) {
        let old_zoom = self._zoom;

        self._step += match zoom {
            ZoomDirection::In => 1.,
            ZoomDirection::Out => -1.,
        };

        self._step = clamp(self._step, 0., Self::ZOOM_STEPS - 1.);

        let ln_min_zoom = f64::ln(Self::MIN_ZOOM);
        let ln_max_zoom = f64::ln(Self::MAX_ZOOM);

        self._zoom = f64::exp(
            ln_min_zoom + (ln_max_zoom - ln_min_zoom) * self._step / (Self::ZOOM_STEPS - 1.0),
        );

        self.x -= (x / old_zoom) - (x / self._zoom);
        self.y -= (y / old_zoom) - (y / self._zoom);
    }

    pub fn zoom(&self) -> f64 {
        self._zoom
    }
}
