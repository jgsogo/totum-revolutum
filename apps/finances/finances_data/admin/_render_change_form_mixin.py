class RenderChangeFormMixin:
    def render_change_form(self, request, context, *args, **kwargs):
        self.change_form_template = "admin/finances_data/change_form.html"
        extra = {"help_text": self.change_form_help_text}

        context.update(extra)
        return super().render_change_form(request, context, *args, **kwargs)
