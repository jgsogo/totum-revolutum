def test_static_files(session):
    """Test that nginx is configured to serve static files"""
    r = session.get("/static/admin/css/base.css")
    assert r.status_code == 200

    r = session.get("/static/flags/es.gif")
    assert r.status_code == 200


def test_app(session):
    """Test admin interface is working"""
    r = session.get("/admin/")
    assert r.status_code == 200, r.text
